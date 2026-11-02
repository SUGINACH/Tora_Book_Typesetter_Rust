'use strict';

const fs = require('node:fs');
const path = require('node:path');
const http = require('node:http');
const crypto = require('node:crypto');
const { execFileSync } = require('node:child_process');
const { chromium } = require(path.resolve(__dirname, '../../legacy/imud/node_modules/playwright'));

const root = path.resolve(__dirname, '../..');
const legacyRoot = path.join(root, 'legacy', 'imud');
const expected = {
  sourceCommit: '6dd79d0ba224c169b0d19778ba181fc90edc8319',
  inputPath: 'text_files_for_typesetter_tests/very_long_test_text.txt',
  inputSha256: '7919f48b14779d3fb98fd19b62388b7b772e1f977c00229217dc481af5529419',
  styleCssSha256: '6d0940914d1626f69f6b27a28e30580d0db4a5fdda0c9d06fae7c103cacc217d',
  sourceFiles: {
    'app.js': '47ba361828ca3afdb7e07cfd88ec39bd134700268b95561489dd496b1b6172a9',
    'decorations.js': '593181e11a51cd0ee6a1afb03250a6fe5d59d5dd53f3b768b6cbd36f47d9166a',
    'index.html': '3f57f1a3ddf5746ff8569e7196a52a22004b8588e77038154716c0273bc1daf8',
    'typesetter.js': '0bf4ecb3db4c247470d884752640287a5c44db215ba6997a4dc0b16543671534'
  },
  settings: [
    {
      id: 'v20x-default',
      path: 'default_settings.json',
      sha256: '5ec4f40f41de5f1c8cbfcaa942b8703a0311cd30610c4346a22ab9f8d0efcc17'
    },
    {
      id: 'v20.5-makom-bina',
      path: 'presets/makom-bina-v20.5/default_settings.json',
      sha256: '319af6a7838aad1487bab0448cf55ef94d63e6549b1a3c0bb1b046ebd6467979'
    }
  ],
  fontCss: [
    { path: 'fonts-def.css', sha256: 'e8739f6aad0cff6aadae6faec39d4aecada85b2303d7548b854a3d7d77bfc48f' },
    { path: 'fonts-extended.css', sha256: '1d038492d7ded85b5967f3f5bb83b56eb5b2c3b900c0e72ce70adf31e62da277' }
  ],
  fontSourceCommit: '0b0ee279464a3db14672cd3f4051275aba5985da',
  fontSourceCssSha256: 'eec01e2b2a65ab67ad959625dd7b3640dceeb09a5cee8b7677787a1e89acd229',
  playwrightVersion: '1.55.0',
  chromiumVersion: '140.0.7339.16',
  viewport: { width: 1500, height: 1500 },
  checkerPath: 'tests/layout/check_layout.js'
};

function sha256(bytes) {
  return crypto.createHash('sha256').update(bytes).digest('hex');
}
function sha256File(file) {
  return sha256(fs.readFileSync(file));
}
function assert(condition, message) {
  if (!condition) throw new Error('IMUD V20.x full-corpus measurement failed: ' + message);
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
  assert(matches.length === 1, 'expected exactly one Google Fonts @import in IMUD style.css');
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

async function runOnce(browser, baseUrl, text, settings, checker, variant, attempt) {
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
    await page.goto(baseUrl + '/index.html', { waitUntil: 'load', timeout: 60000 });
    await page.waitForFunction(() => window.__appReady === true, null, { timeout: 120000 });

    const appliedSettings = await page.evaluate(settings => {
      if (typeof window.applySettingsToDOM !== 'function') throw new Error('applySettingsToDOM is unavailable');
      window.applySettingsToDOM(settings, false);
      if (typeof window.updateGrid === 'function') window.updateGrid();
      if (typeof window.updateStyles === 'function') window.updateStyles();
      const missing = [], mismatches = [];
      for (const [id, expectedValue] of Object.entries(settings)) {
        const element = document.getElementById(id);
        if (!element) { missing.push(id); continue; }
        const actualValue = element.type === 'checkbox' ? !!element.checked : element.value;
        const normalized = element.type === 'checkbox' ? !!expectedValue : String(expectedValue);
        if (actualValue !== normalized) mismatches.push({ id, expected: normalized, actual: actualValue });
      }
      return {
        settingsKeys: Object.keys(settings).length,
        appliedKeys: Object.keys(settings).length - missing.length - mismatches.length,
        missing,
        mismatches
      };
    }, settings);

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

    await page.evaluate(async text => {
      const input = document.getElementById('raw-input');
      if (!input) throw new Error('#raw-input not found');
      if ('value' in input) input.value = text;
      else input.textContent = text;
      let renderResult;
      if (typeof typesetDocumentNow === 'function') renderResult = typesetDocumentNow();
      else if (typeof performTypeset === 'function') renderResult = performTypeset();
      else if (typeof typesetDocument === 'function') renderResult = typesetDocument(0);
      else throw new Error('no supported typeset entry point');
      if (renderResult && typeof renderResult.then === 'function') await renderResult;
    }, text);

    let lastPages = -1, stable = 0, pageCount = 0;
    for (let i = 0; i < 6000; i++) {
      await page.waitForTimeout(150);
      pageCount = await page.evaluate(() => document.querySelectorAll('#book-container .a4-page').length);
      if (pageCount > 0 && pageCount === lastPages) stable++;
      else stable = 0;
      lastPages = pageCount;
      if (stable >= 3) break;
    }
    assert(pageCount > 0 && stable >= 3, 'full-corpus page count did not stabilize');

    await page.addScriptTag({ content: checker });
    const audit = await page.evaluate(() => {
      const issues = window.__checkLayout({});
      const tokens = typeof globalTokens !== 'undefined' && Array.isArray(globalTokens) ? globalTokens : [];
      const paragraphs = tokens.filter(token => token.type === 'p');
      const counts = issues.reduce((result, issue) => {
        result[issue.type] = (result[issue.type] || 0) + 1;
        return result;
      }, {});
      const pages = Array.from(document.querySelectorAll('#book-container .a4-page'));
      const round = value => Math.round(value * 64) / 64;
      const geometry = pages.map((page, pageIndex) => {
        const pageRect = page.getBoundingClientRect();
        const selectors = '.block-2col,.block-single-col,.block-h3-span,.right-col,.left-col,.single-wide-col,.footnote-area,.page-number-footer';
        const nodes = Array.from(page.querySelectorAll(selectors)).map(element => {
          const rect = element.getBoundingClientRect();
          return {
            tag: element.tagName,
            className: typeof element.className === 'string' ? element.className : '',
            x: round(rect.left - pageRect.left),
            y: round(rect.top - pageRect.top),
            width: round(rect.width),
            height: round(rect.height)
          };
        });
        return { pageIndex, nodes };
      });
      return {
        issues,
        counts,
        tokenCount: tokens.length,
        paragraphCount: paragraphs.length,
        missingText: issues.filter(issue => issue.type === 'TEXT_MISSING').length,
        mismatchedText: issues.filter(issue => issue.type === 'TEXT_MISMATCH').length,
        pageCount: pages.length,
        geometry
      };
    });
    const geometrySha256 = sha256(Buffer.from(JSON.stringify(audit.geometry), 'utf8'));
    const signature = sha256(JSON.stringify({
      pageCount: audit.pageCount,
      geometrySha256,
      issues: audit.issues,
      tokenCount: audit.tokenCount,
      paragraphCount: audit.paragraphCount
    }));
    const reportPath = path.join(root, 'artifacts', 'imud-v20x-' + variant + '-attempt-' + attempt + '-layout-report.json');
    fs.mkdirSync(path.dirname(reportPath), { recursive: true });
    fs.writeFileSync(reportPath, JSON.stringify({
      pageCount: audit.pageCount,
      counts: audit.counts,
      issues: audit.issues,
      tokenCount: audit.tokenCount,
      paragraphCount: audit.paragraphCount,
      missingText: audit.missingText,
      mismatchedText: audit.mismatchedText,
      geometrySha256
    }, null, 2) + '\n', 'utf8');

    return {
      attempt,
      pageCount: audit.pageCount,
      issueCount: audit.issues.length,
      issueCounts: audit.counts,
      tokenCount: audit.tokenCount,
      paragraphCount: audit.paragraphCount,
      missingText: audit.missingText,
      mismatchedText: audit.mismatchedText,
      geometrySha256,
      signature,
      settingsApplied,
      fontPreflight,
      reportPath: relative(reportPath),
      reportSha256: sha256File(reportPath),
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
  assert(sourceCommit === expected.sourceCommit, 'IMUD source checkout differs from exact pin');
  const inputPath = path.join(legacyRoot, expected.inputPath);
  assert(sha256File(inputPath) === expected.inputSha256, 'IMUD full-corpus SHA-256 mismatch');
  for (const [sourcePath, hash] of Object.entries(expected.sourceFiles)) {
    assert(sha256File(path.join(legacyRoot, sourcePath)) === hash, 'IMUD source file hash mismatch: ' + sourcePath);
  }
  for (const font of expected.fontCss) {
    assert(sha256File(path.join(legacyRoot, font.path)) === font.sha256, 'IMUD font CSS hash mismatch: ' + font.path);
  }
  const playwrightPackage = JSON.parse(fs.readFileSync(path.join(legacyRoot, 'node_modules', 'playwright', 'package.json'), 'utf8'));
  assert(playwrightPackage.version === expected.playwrightVersion, 'Playwright version mismatch: ' + playwrightPackage.version);
  const browser = await chromium.launch({ headless: true });
  const chromiumVersion = browser.version();
  assert(chromiumVersion === expected.chromiumVersion, 'Chromium version mismatch: ' + chromiumVersion);

  const offlineFonts = prepareOfflineFonts();
  const checkerPath = path.join(legacyRoot, expected.checkerPath);
  const checkerSha256 = sha256File(checkerPath);
  const text = fs.readFileSync(inputPath, 'utf8');
  const { server, url } = await startServer();
  const variants = [];
  try {
    for (const setting of expected.settings) {
      const settingsPath = path.join(legacyRoot, setting.path);
      assert(sha256File(settingsPath) === setting.sha256, 'IMUD settings SHA-256 mismatch: ' + setting.path);
      const settings = JSON.parse(fs.readFileSync(settingsPath, 'utf8'));
      const attempts = [];
      for (let attempt = 1; attempt <= 2; attempt++) {
        attempts.push(await runOnce(browser, url, text, settings, checker, setting.id, attempt));
      }
      const deterministic = attempts[0].signature === attempts[1].signature;
      const settingsApplied = attempts.every(item =>
        item.settingsApplied.missing.length === 0 && item.settingsApplied.mismatches.length === 0);
      const fontsLoaded = attempts.every(item =>
        item.fontPreflight.requested > 0 &&
        item.fontPreflight.loaded === item.fontPreflight.requested &&
        item.fontPreflight.failed.length === 0);
      const offline = attempts.every(item =>
        item.externalRequests.length === 0 && item.pageErrors.length === 0 &&
        item.consoleErrors.length === 0 && item.httpErrors.length === 0);
      const measured = attempts.every(item => item.pageCount > 0) && deterministic &&
        settingsApplied && fontsLoaded && offline;
      variants.push({
        id: setting.id,
        settingsPath: setting.path,
        settingsSha256: setting.sha256,
        status: measured ? 'measured-full-corpus' : 'blocked-settings-font-network-or-browser-gate',
        measured: {
          pages: attempts[0].pageCount,
          issueCount: attempts[0].issueCount,
          issueCounts: attempts[0].issueCounts,
          tokenCount: attempts[0].tokenCount,
          paragraphCount: attempts[0].paragraphCount,
          missingText: attempts[0].missingText,
          mismatchedText: attempts[0].mismatchedText,
          geometrySha256: attempts[0].geometrySha256,
          deterministicTwoFullCorpusRenders: deterministic,
          settingsApplied: attempts[0].settingsApplied,
          fontPreflight: attempts[0].fontPreflight,
          externalRequestCount: attempts.reduce((sum, item) => sum + item.externalRequests.length, 0),
          pageErrorCount: attempts.reduce((sum, item) => sum + item.pageErrors.length, 0),
          consoleErrorCount: attempts.reduce((sum, item) => sum + item.consoleErrors.length, 0),
          httpErrorCount: attempts.reduce((sum, item) => sum + item.httpErrors.length, 0),
          reports: attempts.map(item => ({ path: item.reportPath, sha256: item.reportSha256 })),
          attemptSignatures: attempts.map(item => item.signature)
        }
      });
    }
  } finally {
    server.close();
    await browser.close();
  }

  const complete = variants.every(variant => variant.status === 'measured-full-corpus');
  const manifest = {
    schemaVersion: 1,
    status: complete ? 'measured-all-configurations' : 'blocked-one-or-more-configurations',
    fullCorpusComplete: variants.every(variant => variant.measured.pages > 0),
    runner: 'tools/legacy-runner/run-imud-v20x-full-corpus.cjs',
    targetRustCommit: process.env.RUST_TARGET_COMMIT || process.env.GITHUB_SHA || null,
    legacy: {
      repository: 'SUGINACH/IMUD_TORANI',
      profile: 'legacy-imud-v20x',
      commit: sourceCommit,
      inputPath: expected.inputPath,
      inputSha256: sha256File(inputPath),
      sourceFiles: Object.fromEntries(Object.entries(expected.sourceFiles).map(([file, hash]) => [file, { sha256: hash }])),
      checkerPath: expected.checkerPath,
      checkerSha256: checkerSha256,
      fontCss: expected.fontCss,
      offlineFonts
    },
    environment: {
      node: process.version,
      playwright: playwrightPackage.version,
      chromium: chromiumVersion,
      viewport: expected.viewport,
      network: 'offline-enforced; only local renderer origin, data:, blob: and about: are allowed',
      checkerToleranceCssPx: 1.0,
      timeoutMs: 900000
    },
    variants
  };
  const artifactDir = path.join(root, 'artifacts');
  fs.mkdirSync(artifactDir, { recursive: true });
  const manifestPath = path.join(artifactDir, 'legacy-imud-v20x-full-corpus-manifest.json');
  fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n', 'utf8');
  process.stdout.write('IMUD V20.x full-corpus status: ' + manifest.status + '\n');
  process.stdout.write('Manifest: ' + relative(manifestPath) + '\n');
  assert(complete, 'IMUD V20.x full-corpus measurements did not pass every configuration gate');
}

main().catch(error => {
  process.stderr.write(String(error && (error.stack || error)) + '\n');
  process.exitCode = 1;
});
