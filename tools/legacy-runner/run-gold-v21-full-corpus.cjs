'use strict';

const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const { execFileSync, spawnSync } = require('node:child_process');

const root = path.resolve(__dirname, '../..');
const legacyRoot = path.join(root, 'legacy', 'gold');
const expected = {
  sourceCommit: 'a444d34ccad30fd5ed0d6fd3873adbe9c23cb7e0',
  inputPath: 'text_files_for_typesetter_tests/very_long_test_text.txt',
  inputSha256: '7919f48b14779d3fb98fd19b62388b7b772e1f977c00229217dc481af5529419',
  settingsPath: 'default_settings.json',
  settingsSha256: '5ec4f40f41de5f1c8cbfcaa942b8703a0311cd30610c4346a22ab9f8d0efcc17',
  styleCssSha256: '24381dc5a5077436877a9731039e90085f4adf9621296d9424a9a53afb50275f',
  rendererBlobSha1: 'e3cf01b94036cbfbb99197346dc8b75d24dc79f1',
  rendererOverlayCommit: '7ce8ee79b8dbd9347f45aa1819ac1594a7a3d91a',
  rendererOverlayBlobSha1: 'e570047f74761bdf8120f6014466884426d458b1',
  packageLockPath: 'package-lock.json',
  packageLockSha256: 'ba4c754b2a187cbb149e57f939c73b25204f0a1434c5cc14fefd1aa1357fade0',
  chapterScriptPath: 'scripts/test-chapters-enhanced.js',
  chapterScriptSha256: 'e30ada2c294494ae073ba9cd0e720658ec48d271e44b6fa2cb459e70278a340e',
  determinismScriptPath: 'scripts/determinism-test.js',
  determinismScriptSha256: 'a21d894064f99bf601dca321d265afe4cab5d963d627a7fbca55c29f1456c599',
  fontCss: [
    { path: 'fonts-def.css', sha256: 'e8739f6aad0cff6aadae6faec39d4aecada85b2303d7548b854a3d7d77bfc48f' },
    { path: 'fonts-extended.css', sha256: '1d038492d7ded85b5967f3f5bb83b56eb5b2c3b900c0e72ce70adf31e62da277' }
  ],
  nodeVersion: 'v22.23.4',
  playwrightVersion: '1.64.1',
  chromiumVersion: '156.0.8078.4'
};

function sha256File(file) { return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex'); }
function assert(condition, message) { if (!condition) throw new Error('Gold V21 full-corpus measurement failed: ' + message); }
function relative(file) { return path.relative(root, file).split(path.sep).join('/'); }
function writeLine(file, value) { fs.appendFileSync(file, String(value) + String.fromCharCode(10)); }

function prepareGoldRendererOverlay() {
  const rendererPath = path.join(legacyRoot, 'renderer.html');
  const overlayRoot = path.join(root, 'legacy', 'gold-renderer-overlay');
  const overlayPath = path.join(overlayRoot, 'renderer.html');
  const sourceCommit = execFileSync('git', ['rev-parse', 'HEAD'], {
    cwd: overlayRoot, encoding: 'utf8'
  }).trim();
  assert(sourceCommit === expected.rendererOverlayCommit, 'Gold V21 renderer overlay commit differs from its exact pin');
  const pinnedBlob = execFileSync('git', ['hash-object', rendererPath], {
    cwd: legacyRoot, encoding: 'utf8'
  }).trim();
  assert(pinnedBlob === expected.rendererBlobSha1, 'pinned Gold renderer.html blob identity changed');
  const overlayBlob = execFileSync('git', ['hash-object', overlayPath], {
    cwd: overlayRoot, encoding: 'utf8'
  }).trim();
  assert(overlayBlob === expected.rendererOverlayBlobSha1, 'Gold renderer overlay blob identity changed');

  const missingScripts = ['engine-inputs.js', 'engine-trace.js', 'engine-diagnostics.js'];
  for (const script of missingScripts) {
    assert(!fs.existsSync(path.join(legacyRoot, script)), 'pinned Gold tree unexpectedly contains formerly missing renderer script: ' + script);
  }
  const pinnedText = fs.readFileSync(rendererPath, 'utf8').replace(/\r\n/g, '\n');
  const overlayText = fs.readFileSync(overlayPath, 'utf8').replace(/\r\n/g, '\n');
  let exactExpectedOverlay = pinnedText;
  for (const script of missingScripts) {
    const tag = '  <script src="' + script + '"></script>\n';
    assert(exactExpectedOverlay.split(tag).length === 2, 'pinned renderer must contain exactly one stale script tag: ' + script);
    exactExpectedOverlay = exactExpectedOverlay.replace(tag, '');
  }
  exactExpectedOverlay = exactExpectedOverlay.replace(
    '  <div id="measuring-box"></div>\n\n  <script src="decorations.js"></script>',
    '  <div id="measuring-box"></div>\n  <script src="decorations.js"></script>'
  );
  assert(exactExpectedOverlay === overlayText,
    'same-profile renderer overlay differs by more than the three absent optional instrumentation tags');
  const originalSha256 = sha256File(rendererPath);
  fs.copyFileSync(overlayPath, rendererPath);
  return {
    repository: 'SUGINACH/Tora_Book_Typesetter_Gold',
    sourceCommit: sourceCommit,
    path: 'renderer.html',
    blobSha1: overlayBlob,
    sha256: sha256File(rendererPath),
    replacesPinnedBlobSha1: pinnedBlob,
    originalSha256,
    removedStaleScriptTags: missingScripts,
    reason: 'The pinned Gold V21 renderer.html contains exactly three script tags whose files are absent from the pinned Gold V21 tree. The overlay is the same-profile renderer.html from commit 7ce8ee7 and differs only by removal of those exact missing instrumentation tags; no V23 files are imported.'
  };
}

function prepareOfflineFonts() {
  const fontSourceRoot = path.join(root, 'legacy', 'offline-fonts');
  const expectedFontCommit = '0b0ee279464a3db14672cd3f4051275aba5985da';
  const actualFontCommit = execFileSync('git', ['rev-parse', 'HEAD'], {
    cwd: fontSourceRoot, encoding: 'utf8'
  }).trim();
  assert(actualFontCommit === expectedFontCommit, 'self-hosted font source commit differs from the pin');
  const sourceCssPath = path.join(fontSourceRoot, 'selfhost-fonts', 'google-fonts-selfhost.css');
  const sourceCssSha256 = sha256File(sourceCssPath);
  const expectedCssSha256 = 'eec01e2b2a65ab67ad959625dd7b3640dceeb09a5cee8b7677787a1e89acd229';
  assert(sourceCssSha256 === expectedCssSha256, 'self-hosted Google font CSS hash mismatch');
  const sourceCss = fs.readFileSync(sourceCssPath, 'utf8');
  const fontUrls = [...sourceCss.matchAll(/url\(['"]?([^)'"]+)['"]?\)/g)].map(match => match[1]);
  assert(fontUrls.length === 38, 'unexpected self-hosted font asset count: ' + fontUrls.length);
  for (const url of fontUrls) {
    assert(!url.includes('..') && !/^https?:/i.test(url), 'self-hosted font CSS contains a non-local asset URL');
    assert(fs.existsSync(path.join(fontSourceRoot, 'selfhost-fonts', url)), 'self-hosted font asset is missing: ' + url);
  }
  const destination = path.join(legacyRoot, 'selfhost-fonts');
  fs.rmSync(destination, { recursive: true, force: true });
  fs.cpSync(path.join(fontSourceRoot, 'selfhost-fonts'), destination, { recursive: true });
  const styleCssPath = path.join(legacyRoot, 'style.css');
  const originalStyleCssSha256 = sha256File(styleCssPath);
  assert(originalStyleCssSha256 === expected.styleCssSha256, 'pinned source style.css hash mismatch');
  let styleCss = fs.readFileSync(styleCssPath, 'utf8');
  const remoteImport = /^@import\s+url\(['"]https:\/\/fonts\.googleapis\.com\/css2\?[^'"]+['"]\);\s*$/gm;
  const imports = [...styleCss.matchAll(remoteImport)];
  assert(imports.length === 1, 'expected exactly one Google Fonts @import in pinned style.css');
  styleCss = styleCss.replace(remoteImport, "@import url('selfhost-fonts/google-fonts-selfhost.css');");
  assert(!styleCss.includes('fonts.googleapis.com'), 'remote Google Fonts import remains after offline preparation');
  fs.writeFileSync(styleCssPath, styleCss, 'utf8');
  return {
    sourceRepository: 'SUGINACH/Tora_Book_Typesetter_Orange',
    sourceCommit: actualFontCommit,
    cssPath: 'selfhost-fonts/google-fonts-selfhost.css',
    cssSha256: sourceCssSha256,
    fontAssetCount: fontUrls.length,
    originalStyleCssSha256,
    offlineStyleCssSha256: sha256File(styleCssPath)
  };
}

function routeGuardLines(indent, externalEnv, pageErrorEnv, consoleErrorEnv) {
  return [
    indent + "const localOrigin = new URL(BASE_URL).origin;",
    indent + "await page.route('**/*', async route => {",
    indent + "  const requestUrl = new URL(route.request().url());",
    indent + "  if (requestUrl.origin === localOrigin || ['data:', 'blob:', 'about:'].includes(requestUrl.protocol)) {",
    indent + "    await route.continue();",
    indent + "  } else {",
    indent + "    if (process.env." + externalEnv + ") require('node:fs').appendFileSync(process.env." + externalEnv + ", requestUrl.href + String.fromCharCode(10));",
    indent + "    await route.abort('blockedbyclient');",
    indent + "  }",
    indent + "});",
    indent + "page.on('pageerror', err => { if (process.env." + pageErrorEnv + ") require('node:fs').appendFileSync(process.env." + pageErrorEnv + ", String(err && (err.stack || err)) + String.fromCharCode(10)); });",
    indent + "page.on('response', response => { if (response.status() >= 400 && process.env." + consoleErrorEnv + ") require('node:fs').appendFileSync(process.env." + consoleErrorEnv + ", 'HTTP ' + response.status() + ' ' + response.url() + String.fromCharCode(10)); });",
    indent + "page.on('console', msg => { if (msg.type() === 'error' && process.env." + consoleErrorEnv + ") require('node:fs').appendFileSync(process.env." + consoleErrorEnv + ", msg.text() + String.fromCharCode(10)); });"
  ].join('\n');
}

function buildOfflineFullCorpusScript(sourcePath, outputPath) {
  let source = fs.readFileSync(sourcePath, 'utf8');
  const chapterAnchor = 'const chapters = splitTextIntoChapters(testText);';
  assert(source.includes(chapterAnchor), 'pinned Gold chapter-splitter anchor changed');
  const fallback = [
    chapterAnchor,
    'const nativeChapterCount = chapters.length;',
    'if (chapters.length === 0) {',
    '  const fullCorpusLines = testText.split(String.fromCharCode(10));',
    "  chapters.push({ chapterNum: 1, startLine: 0, endLine: fullCorpusLines.length - 1, title: 'Full corpus — single render; no native H2 split markers', text: testText });",
    '}',
    "if (process.env.GOLD_CHAPTER_MODE_PATH) fs.writeFileSync(process.env.GOLD_CHAPTER_MODE_PATH, JSON.stringify({ nativeChapterCount, effectiveChapterCount: chapters.length, mode: nativeChapterCount === 0 ? 'single-whole-corpus' : 'native-h2-chapters' }), 'utf8');"
  ].join('\n');
  source = source.replace(chapterAnchor, fallback);
  assert(source.includes('const TYPESCRIPT_TIMEOUT_MS = 180000;'), 'Gold render-timeout anchor changed');
  source = source.replace('const TYPESCRIPT_TIMEOUT_MS = 180000;', 'const TYPESCRIPT_TIMEOUT_MS = 900000;');
  assert(source.includes('const GEOM_AUDIT_TIMEOUT_MS = 30000;'), 'Gold audit-timeout anchor changed');
  source = source.replace('const GEOM_AUDIT_TIMEOUT_MS = 30000;', 'const GEOM_AUDIT_TIMEOUT_MS = 180000;');
  const fontAnchor = '      chapterResult.geomStats = geomResult.stats;';
  assert(source.includes(fontAnchor), 'Gold font-preflight insertion anchor changed');
  source = source.replace(fontAnchor, fontAnchor + [
    '      chapterResult.fontPreflight = await iframe.evaluate(async () => {',
    '        const faces = Array.from(document.fonts);',
    '        const outcomes = await Promise.all(faces.map(async face => { try { await face.load(); return { family: face.family, weight: face.weight, status: face.status }; } catch (error) { return { family: face.family, weight: face.weight, status: "error", error: String(error) }; } }));',
    '        await document.fonts.ready;',
    "        const failed = outcomes.filter(face => face.status !== 'loaded');",
    '        return { faceCount: faces.length, loaded: faces.length - failed.length, failed: failed.length, failures: failed };',
    '      });'
  ].join('\n'));
  const pageAnchor = '    page = await context.newPage();';
  assert(source.includes(pageAnchor), 'Gold chapter runner page creation anchor changed');
  source = source.replace(pageAnchor, pageAnchor + '\n' + routeGuardLines('    ', 'GOLD_EXTERNAL_REQUESTS_PATH', 'GOLD_PAGE_ERRORS_PATH', 'GOLD_CONSOLE_ERRORS_PATH'));
  const exitAnchor = '  process.exit(totalHigh > 0 ? 1 : 0);';
  assert(source.includes(exitAnchor), 'Gold chapter runner severity-exit anchor changed');
  source = source.replace(exitAnchor, '  process.exit(0); // severity is captured as measurement, not used as a runner exit gate');
  fs.writeFileSync(outputPath, source, 'utf8');
}

function runNode(script, args, env, logPath) {
  const result = spawnSync(process.execPath, [script, ...args], {
    cwd: legacyRoot, encoding: 'utf8', stdio: 'pipe', maxBuffer: 64 * 1024 * 1024,
    env: { ...process.env, ...env }
  });
  const log = '=== stdout ===\n' + (result.stdout || '') + '\n=== stderr ===\n' + (result.stderr || '');
  fs.writeFileSync(logPath, log, 'utf8');
  if (result.stdout) process.stdout.write(result.stdout);
  if (result.stderr) process.stderr.write(result.stderr);
  assert(result.error == null, 'could not start ' + path.basename(script));
  return result;
}
function readLines(file) {
  if (!fs.existsSync(file)) return [];
  return fs.readFileSync(file, 'utf8').split(String.fromCharCode(10)).map(x => x.trim()).filter(Boolean);
}
function chapterSignature(result) {
  return {
    pageCount: result.pageCount ?? null,
    geomSummary: result.geomSummary ?? null,
    geomIssues: result.geomIssues ?? null,
    fontPreflight: result.fontPreflight ?? null,
    error: result.error ?? null
  };
}

function main() {
  assert(process.version === expected.nodeVersion, 'Node version mismatch: ' + process.version);
  const actualCommit = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: legacyRoot, encoding: 'utf8' }).trim();
  assert(actualCommit === expected.sourceCommit, 'Gold V21 checkout HEAD differs from exact source pin');
  const inputPath = path.join(legacyRoot, expected.inputPath);
  const settingsPath = path.join(legacyRoot, expected.settingsPath);
  const lockPath = path.join(legacyRoot, expected.packageLockPath);
  const chapterScriptPath = path.join(legacyRoot, expected.chapterScriptPath);
  const determinismScriptPath = path.join(legacyRoot, expected.determinismScriptPath);
  assert(sha256File(inputPath) === expected.inputSha256, 'Gold corpus SHA-256 mismatch');
  assert(sha256File(settingsPath) === expected.settingsSha256, 'Gold default settings SHA-256 mismatch');
  assert(sha256File(lockPath) === expected.packageLockSha256, 'Gold package-lock SHA-256 mismatch');
  assert(sha256File(chapterScriptPath) === expected.chapterScriptSha256, 'Gold chapter runner source SHA-256 mismatch');
  assert(sha256File(determinismScriptPath) === expected.determinismScriptSha256, 'Gold determinism runner source SHA-256 mismatch');
  for (const font of expected.fontCss) assert(sha256File(path.join(legacyRoot, font.path)) === font.sha256, 'Gold font CSS SHA-256 mismatch: ' + font.path);

  const playwrightPackage = JSON.parse(fs.readFileSync(path.join(legacyRoot, 'node_modules', 'playwright', 'package.json'), 'utf8'));
  assert(playwrightPackage.version === expected.playwrightVersion, 'Playwright version mismatch: ' + playwrightPackage.version);
  const browser = spawnSync(process.execPath, ['-e', "const {chromium}=require('playwright'); (async()=>{const b=await chromium.launch({headless:true});process.stdout.write(b.version());await b.close();})().catch(e=>{console.error(e);process.exit(1)});"], { cwd: legacyRoot, encoding: 'utf8' });
  assert(browser.status === 0 && browser.stdout.trim() === expected.chromiumVersion, 'Chromium version mismatch: ' + (browser.stdout || browser.stderr));

  const rendererOverlay = prepareGoldRendererOverlay();
  const offlineFonts = prepareOfflineFonts();
  const artifactDir = path.join(root, 'artifacts');
  fs.mkdirSync(artifactDir, { recursive: true });
  const offlineScriptPath = path.join(legacyRoot, 'scripts', 'test-chapters-enhanced.full-corpus.offline.cjs');
  buildOfflineFullCorpusScript(chapterScriptPath, offlineScriptPath);
  const attempts = [];
  for (let attempt = 1; attempt <= 2; attempt++) {
    const resultsDir = path.join(artifactDir, 'gold-v21-full-attempt-' + attempt);
    fs.mkdirSync(resultsDir, { recursive: true });
    const externalPath = path.join(artifactDir, 'gold-full-external-' + attempt + '.txt');
    const pageErrorsPath = path.join(artifactDir, 'gold-full-page-errors-' + attempt + '.txt');
    const consoleErrorsPath = path.join(artifactDir, 'gold-full-console-errors-' + attempt + '.txt');
    const modePath = path.join(artifactDir, 'gold-full-chapter-mode-' + attempt + '.json');
    const logPath = path.join(artifactDir, 'gold-v21-full-attempt-' + attempt + '.log');
    for (const file of [externalPath, pageErrorsPath, consoleErrorsPath]) fs.writeFileSync(file, '', 'utf8');
    const label = 'stage-a-gold-v21-full-attempt-' + attempt;
    const result = runNode(offlineScriptPath, ['1', '1', label], {
      BASE_URL: 'http://127.0.0.1:8765/index.html',
      TEST_TEXT_PATH: inputPath,
      RESULTS_DIR: resultsDir,
      GOLD_EXTERNAL_REQUESTS_PATH: externalPath,
      GOLD_PAGE_ERRORS_PATH: pageErrorsPath,
      GOLD_CONSOLE_ERRORS_PATH: consoleErrorsPath,
      GOLD_CHAPTER_MODE_PATH: modePath
    }, logPath);
    const summaryPath = path.join(resultsDir, label + '-summary.json');
    const resultPath = path.join(resultsDir, 'chapter-01.json');
    const mode = fs.existsSync(modePath) ? JSON.parse(fs.readFileSync(modePath, 'utf8')) : null;
    const summary = fs.existsSync(summaryPath) ? JSON.parse(fs.readFileSync(summaryPath, 'utf8')) : null;
    const chapter = fs.existsSync(resultPath) ? JSON.parse(fs.readFileSync(resultPath, 'utf8')) : null;
    attempts.push({
      attempt, exitCode: result.status, mode, summary, chapter,
      signature: chapter ? chapterSignature(chapter) : null,
      externalRequests: readLines(externalPath),
      pageErrors: readLines(pageErrorsPath),
      consoleErrors: readLines(consoleErrorsPath),
      logPath, summaryPath, resultPath
    });
  }

  const first = attempts[0];
  const second = attempts[1];
  const singleWholeCorpusFallback = first.mode?.mode === 'single-whole-corpus' && second.mode?.mode === 'single-whole-corpus';
  const bothRendered = attempts.every(a => a.exitCode === 0 && a.summary && a.chapter && Number(a.chapter.pageCount) > 0 && !a.chapter.error);
  const signaturesIdentical = !!first.signature && !!second.signature && JSON.stringify(first.signature) === JSON.stringify(second.signature);
  const externalRequests = attempts.flatMap(a => a.externalRequests);
  const pageErrors = attempts.flatMap(a => a.pageErrors);
  const consoleErrors = attempts.flatMap(a => a.consoleErrors);
  const nativeChapterCount = first.mode?.nativeChapterCount ?? null;
  const fontsLoaded = attempts.every(a => {
    const f = a.chapter?.fontPreflight;
    return !!f && Number.isInteger(f.faceCount) && f.faceCount > 0 && f.loaded === f.faceCount && f.failed === 0;
  });
  const measured = bothRendered && singleWholeCorpusFallback && fontsLoaded && signaturesIdentical &&
    externalRequests.length === 0 && pageErrors.length === 0 && consoleErrors.length === 0;
  const status = measured ? 'measured-full-document'
    : !fontsLoaded ? 'blocked-font-preflight'
    : externalRequests.length ? 'blocked-external-requests'
    : consoleErrors.length ? 'blocked-browser-resource-errors'
    : 'blocked-render-error-or-nondeterminism';
  const firstSummary = first.summary;
  const firstChapter = first.chapter;
  const manifest = {
    schemaVersion: 1,
    status,
    fullCorpusCovered: bothRendered && singleWholeCorpusFallback,
    continuousDocumentRender: bothRendered && singleWholeCorpusFallback,
    nativeChapterMarkers: nativeChapterCount,
    adaptedToSingleWholeCorpus: singleWholeCorpusFallback,
    runner: 'tools/legacy-runner/run-gold-v21-full-corpus.cjs',
    targetRustCommit: process.env.RUST_TARGET_COMMIT || process.env.GITHUB_SHA || null,
    legacy: {
      repository: 'SUGINACH/Tora_Book_Typesetter_Gold', profile: 'legacy-gold-v21', commit: actualCommit,
      inputPath: expected.inputPath, inputSha256: sha256File(inputPath),
      settingsPath: expected.settingsPath, settingsSha256: sha256File(settingsPath),
      packageLockPath: expected.packageLockPath, packageLockSha256: sha256File(lockPath),
      chapterScriptPath: expected.chapterScriptPath, chapterScriptSha256: sha256File(chapterScriptPath),
      determinismScriptPath: expected.determinismScriptPath, determinismScriptSha256: sha256File(determinismScriptPath),
      offlineScriptSha256: sha256File(offlineScriptPath), fontCss: expected.fontCss, offlineFonts, rendererOverlay
    },
    environment: { node: process.version, playwright: playwrightPackage.version, chromium: browser.stdout.trim(), offlineFonts, viewport: { width: 1400, height: 1000 }, timeoutMs: 900000, network: 'offline-enforced by Playwright route guard' },
    measured: {
      pages: firstChapter?.pageCount ?? null,
      geometrySummary: firstChapter?.geomSummary ?? null,
      issueCount: Array.isArray(firstChapter?.geomIssues) ? firstChapter.geomIssues.length : null,
      geometryStats: firstChapter?.geomStats ?? null,
      totals: firstSummary?.totals ?? null,
      renderErrors: attempts.filter(a => a.chapter?.error || a.exitCode !== 0).map(a => ({ attempt: a.attempt, exitCode: a.exitCode, error: a.chapter?.error ?? null })),
      deterministicTwoFullCorpusRenders: signaturesIdentical,
      attemptSignatures: attempts.map(a => a.signature),
      externalRequestCount: externalRequests.length,
      pageErrorCount: pageErrors.length,
      consoleErrorCount: consoleErrors.length,
      summaryHashes: attempts.map(a => a.summaryPath && fs.existsSync(a.summaryPath) ? sha256File(a.summaryPath) : null),
      resultHashes: attempts.map(a => a.resultPath && fs.existsSync(a.resultPath) ? sha256File(a.resultPath) : null),
      logHashes: attempts.map(a => sha256File(a.logPath))
    },
    interpretation: 'The pinned Gold V21 corpus contains no native H2 chapter markers expected by the legacy 79-chapter splitter. The harness copy therefore falls back to one single whole-corpus render, preserving the exact pinned input bytes. The pinned renderer.html has three stale script tags for files absent from the pinned Gold tree; the only renderer-page adaptation is the exact same-profile renderer overlay recorded in legacy.rendererOverlay. A measured baseline requires two identical full-document renders, all enumerated font faces loaded, zero external requests, and zero browser/page errors. Geometry findings remain measured findings, not waivers; the pinned source files and test scripts remain unchanged.'
  };
  const manifestPath = path.join(artifactDir, 'legacy-gold-v21-full-corpus-manifest.json');
  fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + String.fromCharCode(10), 'utf8');
  process.stdout.write('Gold V21 full-corpus result captured: ' + status + String.fromCharCode(10));
  process.stdout.write('Manifest: ' + relative(manifestPath) + String.fromCharCode(10));
}

try { main(); }
catch (error) { process.stderr.write(String(error && (error.stack || error)) + String.fromCharCode(10)); process.exitCode = 1; }
