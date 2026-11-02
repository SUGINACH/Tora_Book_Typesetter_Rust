'use strict';

const fs = require('node:fs');
const path = require('node:path');
const http = require('node:http');
const crypto = require('node:crypto');
const { execFileSync } = require('node:child_process');
const { chromium } = require(path.resolve(__dirname, '../../legacy/yellow/node_modules/playwright'));

const root = path.resolve(__dirname, '../..');
const legacyRoot = path.join(root, 'legacy', 'yellow');
const expected = {
  sourceCommit: '878535e12eb4aa7e3c4a72e0e79687082d8a6d3b',
  inputPath: 'long_test_text.txt',
  inputSha256: '624f70cf78fd09ceb1342bc75ccec11c6dc99fc30bf341c99aafa4e1520f6e64',
  settingsPath: 'default_settings.json',
  settingsSha256: 'ec5d8d98f5c24546605a35c32dca17c4ef8428d506d53aaa65f65cbf47d5dd68',
  styleCssSha256: 'b5abcc624e75b03bc6a4ce0321858f889c833ef0707d5098f4cf091c7c6b66dd',
  auditPath: 'geometry-audit.js',
  auditSha256: '7ff5d1bd2ce5008374856a8948383670384afcd6729b59cfdcc3ae5956a82feb',
  fontCss: [
    { path: 'fonts-def.css', sha256: 'e8739f6aad0cff6aadae6faec39d4aecada85b2303d7548b854a3d7d77bfc48f' },
    { path: 'fonts-extended.css', sha256: '1d038492d7ded85b5967f3f5bb83b56eb5b2c3b900c0e72ce70adf31e62da277' }
  ],
  fontSourceCommit: '0b0ee279464a3db14672cd3f4051275aba5985da',
  fontSourceCssSha256: 'eec01e2b2a65ab67ad959625dd7b3640dceeb09a5cee8b7677787a1e89acd229',
  playwrightVersion: '1.55.0',
  chromiumVersion: '140.0.7339.16',
  viewport: { width: 1280, height: 900 }
};

function sha256(bytes) {
  return crypto.createHash('sha256').update(bytes).digest('hex');
}
function sha256File(file) {
  return sha256(fs.readFileSync(file));
}
function assert(condition, message) {
  if (!condition) throw new Error('Yellow V10 full-corpus measurement failed: ' + message);
}
function relative(file) {
  return path.relative(root, file).split(path.sep).join('/');
}

function prepareOfflineFonts() {
  const fontSourceRoot = path.join(root, 'legacy', 'offline-fonts');
  const actualCommit = execFileSync('git', ['rev-parse', 'HEAD'], {
    cwd: fontSourceRoot, encoding: 'utf8'
  }).trim();
  assert(actualCommit === expected.fontSourceCommit, 'self-hosted font source commit differs from the pin');
  const sourceCssPath = path.join(fontSourceRoot, 'selfhost-fonts', 'google-fonts-selfhost.css');
  assert(sha256File(sourceCssPath) === expected.fontSourceCssSha256, 'self-hosted Google font CSS hash mismatch');
  const sourceCss = fs.readFileSync(sourceCssPath, 'utf8');
  const fontUrls = [...sourceCss.matchAll(/url\(['"]?([^)'"]+)['"]?\)/g)].map(match => match[1]);
  assert(fontUrls.length === 38, 'unexpected self-hosted font asset count: ' + fontUrls.length);
  for (const url of fontUrls) {
    assert(!url.includes('..') && !/^https?:/i.test(url), 'non-local font URL in self-hosted CSS');
    assert(fs.existsSync(path.join(fontSourceRoot, 'selfhost-fonts', url)), 'missing self-hosted font asset: ' + url);
  }
  const destination = path.join(legacyRoot, 'selfhost-fonts');
  fs.rmSync(destination, { recursive: true, force: true });
  fs.cpSync(path.join(fontSourceRoot, 'selfhost-fonts'), destination, { recursive: true });

  const stylePath = path.join(legacyRoot, 'style.css');
  const originalStyleCssSha256 = sha256File(stylePath);
  assert(originalStyleCssSha256 === expected.styleCssSha256, 'pinned source style.css hash mismatch');
  let style = fs.readFileSync(stylePath, 'utf8');
  const remoteImport = /^@import\s+url\(['"]https:\/\/fonts\.googleapis\.com\/css2\?[^'"]+['"]\);\s*$/gm;
  const matches = [...style.matchAll(remoteImport)];
  assert(matches.length === 1, 'expected exactly one Google Fonts @import in Yellow style.css');
  style = style.replace(remoteImport, "@import url('selfhost-fonts/google-fonts-selfhost.css');");
  assert(!style.includes('fonts.googleapis.com'), 'remote Google Fonts import remains');
  fs.writeFileSync(stylePath, style, 'utf8');

  return {
    repository: 'SUGINACH/Tora_Book_Typesetter_Orange',
    commit: actualCommit,
    cssPath: 'selfhost-fonts/google-fonts-selfhost.css',
    cssSha256: expected.fontSourceCssSha256,
    fontAssetCount: fontUrls.length,
    originalStyleCssSha256,
    offlineStyleCssSha256: sha256File(stylePath)
  };
}

function startServer() {
  const mime = {
    '.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8',
    '.css': 'text/css; charset=utf-8', '.json': 'application/json; charset=utf-8',
    '.woff2': 'font/woff2', '.woff': 'font/woff', '.ttf': 'font/ttf',
    '.svg': 'image/svg+xml', '.png': 'image/png', '.ico': 'image/x-icon',
    '.txt': 'text/plain; charset=utf-8'
  };
  const server = http.createServer((req, res) => {
    let pathname;
    try { pathname = decodeURIComponent(new URL(req.url, 'http://127.0.0.1').pathname); }
    catch { res.writeHead(400); res.end(); return; }
    const file = path.resolve(legacyRoot, '.' + pathname);
    if (file !== legacyRoot && !file.startsWith(legacyRoot + path.sep)) {
      res.writeHead(403); res.end(); return;
    }
    let target = file;
    if (fs.existsSync(target) && fs.statSync(target).isDirectory()) target = path.join(target, 'index.html');
    fs.readFile(target, (error, bytes) => {
      if (error) { res.writeHead(404); res.end('not found'); return; }
      res.writeHead(200, { 'content-type': mime[path.extname(target).toLowerCase()] || 'application/octet-stream' });
      res.end(bytes);
    });
  });
  return new Promise(resolve => server.listen(0, '127.0.0.1', () => {
    resolve({ server, url: 'http://127.0.0.1:' + server.address().port });
  }));
}

function familiesFromSettings(settings) {
  const families = new Set();
  for (const [key, value] of Object.entries(settings)) {
    if (typeof value === 'string' && /^(font-|f-)/.test(key)) {
      for (const match of value.matchAll(/'([^']+)'/g)) families.add(match[1]);
    }
  }
  return [...families].sort();
}

async function runOnce(browser, baseUrl, text, settings, auditApi, attempt) {
  const context = await browser.newContext({ viewport: expected.viewport });
  const page = await context.newPage();
  const externalRequests = [];
  const pageErrors = [];
  const consoleErrors = [];
  const httpErrors = [];
  const localOrigin = new URL(baseUrl).origin;

  await page.route('**/*', async route => {
    const url = new URL(route.request().url());
    if (url.origin === localOrigin || ['data:', 'blob:', 'about:'].includes(url.protocol)) {
      await route.continue();
    } else {
      externalRequests.push(url.href);
      await route.abort('blockedbyclient');
    }
  });
  page.on('pageerror', error => pageErrors.push(String(error && (error.stack || error))));
  page.on('console', message => {
    if (message.type() === 'error') consoleErrors.push(message.text());
  });
  page.on('response', response => {
    if (response.status() >= 400) httpErrors.push({ status: response.status(), url: response.url() });
  });

  try {
    await page.goto(baseUrl + '/renderer.html', { waitUntil: 'load', timeout: 60000 });
    await page.waitForFunction(() => typeof window.typesetDocument === 'function' && !!window.__geomAudit,
      null, { timeout: 60000 });

    const requestedFamilies = familiesFromSettings(settings);
    const fontPreflight = await page.evaluate(async families => {
      const results = await Promise.all(families.map(async family => {
        try {
          const loadedFaces = await document.fonts.load('16px "' + family.replace(/"/g, '') + '"');
          const checked = document.fonts.check('16px "' + family.replace(/"/g, '') + '"');
          return { family, loadedFaceCount: loadedFaces.length, loaded: loadedFaces.length > 0 && checked };
        } catch (error) {
          return { family, loaded: false, error: String(error) };
        }
      }));
      await document.fonts.ready;
      return {
        requested: results.length,
        loaded: results.filter(item => item.loaded).length,
        failed: results.filter(item => !item.loaded),
        results
      };
    }, requestedFamilies);

    const render = await page.evaluate(({ text, settings }) => new Promise(resolve => {
      const timer = setTimeout(() => resolve({ type: 'timeout', error: 'render-done timeout' }), 900000);
      const handler = event => {
        if (!event.data || !['render-done', 'render-error'].includes(event.data.type)) return;
        window.removeEventListener('message', handler);
        clearTimeout(timer);
        resolve(event.data);
      };
      window.addEventListener('message', handler);
      window.postMessage({ type: 'typeset', text, settings }, '*');
    }), { text, settings });
    assert(render.type === 'render-done', 'render failed on attempt ' + attempt + ': ' + JSON.stringify(render).slice(0, 500));
    await page.waitForTimeout(500);

    const model = await page.evaluate(() => {
      window.scrollTo({ top: 0, behavior: 'instant' });
      const extracted = window.__geomAudit.extract();
      const strip = node => {
        delete node.words;
        (node.children || []).forEach(strip);
        return node;
      };
      return {
        pages: extracted.pages.map(pageNode => strip(window.__geomAudit.cleanForExport(pageNode))),
        stats: extracted.stats
      };
    });
    const audit = auditApi.audit(model);
    const reportPath = path.join(root, 'artifacts', 'yellow-v10-attempt-' + attempt + '-geometry-report.json');
    fs.mkdirSync(path.dirname(reportPath), { recursive: true });
    fs.writeFileSync(reportPath, JSON.stringify({
      pageCount: render.pageCount,
      modelStats: model.stats,
      modelSha256: sha256(Buffer.from(JSON.stringify(model), 'utf8')),
      summary: audit.summary,
      issues: audit.issues
    }, null, 2) + '\n', 'utf8');
    const reportSha256 = sha256File(reportPath);
    const modelBytes = Buffer.from(JSON.stringify(model), 'utf8');
    const modelSha256 = sha256(modelBytes);
    const signature = sha256(JSON.stringify({
      pageCount: render.pageCount,
      modelSha256,
      summary: audit.summary,
      issues: audit.issues
    }));
    return {
      attempt,
      pageCount: render.pageCount,
      modelSha256,
      modelBytes: modelBytes.length,
      signature,
      summary: audit.summary,
      issueCount: audit.issues.length,
      reportPath: relative(reportPath),
      reportSha256,
      issueKinds: audit.issues.reduce((result, issue) => {
        result[issue.kind] = (result[issue.kind] || 0) + 1;
        return result;
      }, {}),
      fontPreflight,
      externalRequests,
      pageErrors,
      consoleErrors,
      httpErrors
    };
  } finally {
    await context.close();
  }
}

async function main() {
  assert(process.version === 'v22.23.4', 'Node version mismatch: ' + process.version);
  const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: legacyRoot, encoding: 'utf8' }).trim();
  assert(sourceCommit === expected.sourceCommit, 'Yellow source checkout differs from exact pin');

  const inputPath = path.join(legacyRoot, expected.inputPath);
  const settingsPath = path.join(legacyRoot, expected.settingsPath);
  const auditPath = path.join(legacyRoot, expected.auditPath);
  assert(sha256File(inputPath) === expected.inputSha256, 'Yellow corpus SHA-256 mismatch');
  assert(sha256File(settingsPath) === expected.settingsSha256, 'Yellow settings SHA-256 mismatch');
  assert(sha256File(auditPath) === expected.auditSha256, 'Yellow geometry-audit.js SHA-256 mismatch');
  for (const font of expected.fontCss) {
    assert(sha256File(path.join(legacyRoot, font.path)) === font.sha256, 'Yellow font CSS hash mismatch: ' + font.path);
  }
  const playwrightPackage = JSON.parse(fs.readFileSync(path.join(legacyRoot, 'node_modules', 'playwright', 'package.json'), 'utf8'));
  assert(playwrightPackage.version === expected.playwrightVersion, 'Playwright version mismatch: ' + playwrightPackage.version);
  const browser = await chromium.launch({ headless: true });
  const chromiumVersion = browser.version();
  assert(chromiumVersion === expected.chromiumVersion, 'Chromium version mismatch: ' + chromiumVersion);

  const offlineFonts = prepareOfflineFonts();
  const settings = JSON.parse(fs.readFileSync(settingsPath, 'utf8'));
  const text = fs.readFileSync(inputPath, 'utf8');
  const auditApi = require(path.join(legacyRoot, expected.auditPath));
  const { server, url } = await startServer();
  const attempts = [];
  try {
    for (let attempt = 1; attempt <= 2; attempt++) {
      attempts.push(await runOnce(browser, url, text, settings, auditApi, attempt));
    }
  } finally {
    server.close();
    await browser.close();
  }

  const first = attempts[0];
  const second = attempts[1];
  const deterministic = first.signature === second.signature;
  const noExternal = attempts.every(item => item.externalRequests.length === 0);
  const noBrowserErrors = attempts.every(item =>
    item.pageErrors.length === 0 && item.consoleErrors.length === 0 && item.httpErrors.length === 0);
  const fontsLoaded = attempts.every(item => item.fontPreflight.requested > 0 &&
    item.fontPreflight.loaded === item.fontPreflight.requested && item.fontPreflight.failed.length === 0);
  const complete = attempts.every(item => item.pageCount > 0) && deterministic && noExternal && noBrowserErrors && fontsLoaded;
  const manifest = {
    schemaVersion: 1,
    status: complete ? 'measured-full-corpus' : 'blocked-font-network-or-browser-gate',
    fullCorpusComplete: attempts.every(item => item.pageCount > 0),
    runner: 'tools/legacy-runner/run-yellow-v10-full-corpus.cjs',
    targetRustCommit: process.env.RUST_TARGET_COMMIT || process.env.GITHUB_SHA || null,
    legacy: {
      repository: 'SUGINACH/Tora_Book_Typesetter_Yellow',
      profile: 'legacy-yellow-v10',
      commit: sourceCommit,
      inputPath: expected.inputPath,
      inputSha256: sha256File(inputPath),
      settingsPath: expected.settingsPath,
      settingsSha256: sha256File(settingsPath),
      styleCssSha256: expected.styleCssSha256,
      auditPath: expected.auditPath,
      auditSha256: expected.auditSha256,
      fontCss: expected.fontCss,
      offlineFonts
    },
    environment: {
      node: process.version,
      playwright: playwrightPackage.version,
      chromium: chromiumVersion,
      viewport: expected.viewport,
      network: 'offline-enforced; only local renderer origin, data:, blob: and about: are allowed',
      timeoutMs: 900000
    },
    measured: {
      pages: first.pageCount,
      geometrySummary: first.summary,
      issueCount: first.issueCount,
      issueKinds: first.issueKinds,
      modelSha256: first.modelSha256,
      modelBytes: first.modelBytes,
      deterministicTwoFullCorpusRenders: deterministic,
      attemptSignatures: attempts.map(item => item.signature),
      fontPreflight: first.fontPreflight,
      externalRequestCount: attempts.reduce((sum, item) => sum + item.externalRequests.length, 0),
      pageErrorCount: attempts.reduce((sum, item) => sum + item.pageErrors.length, 0),
      consoleErrorCount: attempts.reduce((sum, item) => sum + item.consoleErrors.length, 0),
      httpErrorCount: attempts.reduce((sum, item) => sum + item.httpErrors.length, 0),
      httpErrors: attempts.flatMap(item => item.httpErrors)
    },
    attempts
  };

  const artifactDir = path.join(root, 'artifacts');
  fs.mkdirSync(artifactDir, { recursive: true });
  const manifestPath = path.join(artifactDir, 'legacy-yellow-v10-full-corpus-manifest.json');
  fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n', 'utf8');
  process.stdout.write('Yellow V10 full-corpus status: ' + manifest.status + '\n');
  process.stdout.write('Manifest: ' + relative(manifestPath) + '\n');
  assert(complete, 'Yellow V10 full-corpus measurement did not pass the environment/determinism gates');
}

main().catch(error => {
  process.stderr.write(String(error && (error.stack || error)) + '\n');
  process.exitCode = 1;
});
