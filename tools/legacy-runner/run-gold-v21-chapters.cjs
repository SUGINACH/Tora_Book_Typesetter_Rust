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
  chromiumVersion: '156.0.8078.4',
  chapterCount: 79
};

function sha256File(file) {
  return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
}
function assert(condition, message) {
  if (!condition) throw new Error('Gold V21 profile measurement failed: ' + message);
}
function relative(file) {
  return path.relative(root, file).split(path.sep).join('/');
}
function routeGuardLines(indent, envName) {
  return [
    indent + "const localOrigin = new URL(BASE_URL).origin;",
    indent + "await page.route('**/*', async route => {",
    indent + "  const requestUrl = new URL(route.request().url());",
    indent + "  if (requestUrl.origin === localOrigin || ['data:', 'blob:', 'about:'].includes(requestUrl.protocol)) {",
    indent + "    await route.continue();",
    indent + "  } else {",
    indent + "    if (process.env." + envName + ") require('node:fs').appendFileSync(process.env." + envName + ", requestUrl.href + String.fromCharCode(10));",
    indent + "    await route.abort('blockedbyclient');",
    indent + "  }",
    indent + "});"
  ].join('\n');
}
function buildOfflineChapterScript(sourcePath, outputPath) {
  let source = fs.readFileSync(sourcePath, 'utf8');
  const pageAnchor = '    page = await context.newPage();';
  assert(source.includes(pageAnchor), 'Gold chapter runner page creation anchor changed');
  source = source.replace(pageAnchor, pageAnchor + '\n' + routeGuardLines('    ', 'GOLD_EXTERNAL_REQUESTS_PATH'));
  const errorAnchor = "      console.error(`  [page-error] ${err.message}`);";
  assert(source.includes(errorAnchor), 'Gold chapter runner page-error anchor changed');
  source = source.replace(errorAnchor, errorAnchor + "\n      if (process.env.GOLD_PAGE_ERRORS_PATH) require('node:fs').appendFileSync(process.env.GOLD_PAGE_ERRORS_PATH, String(err && (err.stack || err)) + String.fromCharCode(10));");
  const exitAnchor = '  process.exit(totalHigh > 0 ? 1 : 0);';
  assert(source.includes(exitAnchor), 'Gold chapter runner exit anchor changed');
  source = source.replace(exitAnchor, '  process.exit(0); // measurement captures severity; it is not a zero-findings gate');
  fs.writeFileSync(outputPath, source, 'utf8');
}
function buildOfflineDeterminismScript(sourcePath, outputPath) {
  let source = fs.readFileSync(sourcePath, 'utf8');
  const pageAnchor = '    const page = await browser.newPage();';
  assert(source.includes(pageAnchor), 'Gold determinism runner page creation anchor changed');
  source = source.replace(pageAnchor, pageAnchor + '\n' + routeGuardLines('    ', 'GOLD_EXTERNAL_REQUESTS_PATH') + '\n    page.on(\'pageerror\', err => { if (process.env.GOLD_PAGE_ERRORS_PATH) require(\'node:fs\').appendFileSync(process.env.GOLD_PAGE_ERRORS_PATH, String(err && (err.stack || err)) + String.fromCharCode(10)); });');
  fs.writeFileSync(outputPath, source, 'utf8');
}
function runNode(script, args, env, logPath) {
  const result = spawnSync(process.execPath, [script, ...args], {
    cwd: legacyRoot,
    encoding: 'utf8',
    stdio: 'pipe',
    maxBuffer: 64 * 1024 * 1024,
    env: { ...process.env, ...env }
  });
  const log = '=== stdout ===\n' + (result.stdout || '') + '\n=== stderr ===\n' + (result.stderr || '');
  fs.writeFileSync(logPath, log, 'utf8');
  process.stdout.write(result.stdout || '');
  if (result.stderr) process.stderr.write(result.stderr);
  assert(result.error == null, 'could not start ' + path.basename(script));
  return result;
}
function readLines(file) {
  if (!fs.existsSync(file)) return [];
  return fs.readFileSync(file, 'utf8').split(String.fromCharCode(10)).map(x => x.trim()).filter(Boolean);
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

  const artifactDir = path.join(root, 'artifacts');
  const resultsDir = path.join(artifactDir, 'gold-v21-chapter-results');
  fs.mkdirSync(resultsDir, { recursive: true });
  const externalRequestsPath = path.join(artifactDir, 'gold-external-requests.txt');
  const pageErrorsPath = path.join(artifactDir, 'gold-page-errors.txt');
  const chapterLogPath = path.join(artifactDir, 'gold-v21-chapters.log');
  const determinismLogPath = path.join(artifactDir, 'gold-v21-determinism.log');
  fs.writeFileSync(externalRequestsPath, '', 'utf8');
  fs.writeFileSync(pageErrorsPath, '', 'utf8');

  const offlineChapterPath = path.join(legacyRoot, 'scripts', 'test-chapters-enhanced.offline.cjs');
  const offlineDeterminismPath = path.join(legacyRoot, 'scripts', 'determinism-test.offline.cjs');
  buildOfflineChapterScript(chapterScriptPath, offlineChapterPath);
  buildOfflineDeterminismScript(determinismScriptPath, offlineDeterminismPath);

  const commonEnv = {
    BASE_URL: 'http://127.0.0.1:8765/index.html',
    TEST_TEXT_PATH: inputPath,
    GOLD_EXTERNAL_REQUESTS_PATH: externalRequestsPath,
    GOLD_PAGE_ERRORS_PATH: pageErrorsPath
  };
  const full = runNode(offlineChapterPath, ['1', '79', 'stage-a-gold-v21'], { ...commonEnv, RESULTS_DIR: resultsDir }, chapterLogPath);
  assert(full.status === 0, 'Gold chapter harness process failed: ' + full.status);
  const summaryPath = path.join(resultsDir, 'stage-a-gold-v21-summary.json');
  assert(fs.existsSync(summaryPath), 'Gold 79-chapter summary missing');
  const summary = JSON.parse(fs.readFileSync(summaryPath, 'utf8'));
  assert(summary.startChapter === 1 && summary.endChapter === 79, 'Gold summary does not cover chapters 1..79');
  assert(Array.isArray(summary.chapters) && summary.chapters.length === 79, 'Gold summary does not contain exactly 79 chapters');
  const chapterNumbers = summary.chapters.map(x => x.chapterNum).sort((a,b)=>a-b);
  assert(chapterNumbers.every((x,i)=>x===i+1), 'Gold chapter result IDs are missing, duplicated or out of order');
  const totalPages = summary.chapters.reduce((sum, x) => sum + (Number(x.pageCount) || 0), 0);
  const chapterErrors = summary.chapters.filter(x => x.error);
  const totals = summary.totals || {};
  assert(totalPages > 0, 'Gold chapter run produced no pages');
  for (let n = 1; n <= 79; n++) assert(fs.existsSync(path.join(resultsDir, 'chapter-' + String(n).padStart(2, '0') + '.json')), 'Gold chapter result file missing: ' + n);

  fs.writeFileSync(externalRequestsPath, '', 'utf8');
  fs.writeFileSync(pageErrorsPath, '', 'utf8');
  const deterministic = runNode(offlineDeterminismPath, ['9', '3'], commonEnv, determinismLogPath);
  const deterministicText = String(deterministic.stdout || '') + String.fromCharCode(10) + String(deterministic.stderr || '');
  const determinismPass = deterministic.status === 0 && deterministicText.includes('unique page counts: 1') && deterministicText.includes('unique summaries: 1');
  const externalRequests = readLines(externalRequestsPath);
  const pageErrors = readLines(pageErrorsPath);
  assert(externalRequests.length === 0, 'Gold runner attempted external network requests');
  assert(pageErrors.length === 0, 'Gold browser reported page errors');

  const chapterFiles = Array.from({ length: 79 }, (_, i) => path.join(resultsDir, 'chapter-' + String(i + 1).padStart(2, '0') + '.json'));
  const chapterHashes = chapterFiles.map(file => sha256File(file));
  const chapterResultsDigest = crypto.createHash('sha256').update(chapterHashes.join('\n')).digest('hex');
  const status = chapterErrors.length === 0 && determinismPass ? 'measured-chapter-sliced' : 'blocked-chapter-errors-or-nondeterminism';
  const manifest = {
    schemaVersion: 1,
    status,
    fullCorpusCovered: true,
    continuousDocumentRender: false,
    runner: 'tools/legacy-runner/run-gold-v21-chapters.cjs',
    targetRustCommit: process.env.RUST_TARGET_COMMIT || process.env.GITHUB_SHA || null,
    legacy: {
      repository: 'SUGINACH/Tora_Book_Typesetter_Gold',
      profile: 'legacy-gold-v21',
      commit: actualCommit,
      inputPath: expected.inputPath, inputSha256: sha256File(inputPath),
      settingsPath: expected.settingsPath, settingsSha256: sha256File(settingsPath),
      packageLockPath: expected.packageLockPath, packageLockSha256: sha256File(lockPath),
      chapterScriptPath: expected.chapterScriptPath, chapterScriptSha256: sha256File(chapterScriptPath),
      determinismScriptPath: expected.determinismScriptPath, determinismScriptSha256: sha256File(determinismScriptPath),
      offlineChapterScriptSha256: sha256File(offlineChapterPath),
      offlineDeterminismScriptSha256: sha256File(offlineDeterminismPath),
      fontCss: expected.fontCss
    },
    environment: { node: process.version, playwright: playwrightPackage.version, chromium: browser.stdout.trim(), viewport: { width: 1400, height: 1000 }, network: 'offline-enforced by Playwright route guard', chapterRestartInterval: 3 },
    measured: {
      chapters: summary.chapters.length,
      totalPagesAcrossChapterSlices: totalPages,
      high: totals.high ?? null, medium: totals.medium ?? null, low: totals.low ?? null,
      chaptersWithErrors: chapterErrors.length,
      chapterErrors: chapterErrors.map(x => ({ chapterNum: x.chapterNum, error: x.error })),
      deterministicChapter9ThreeRuns: determinismPass,
      deterministicRunnerExitCode: deterministic.status,
      externalRequestCount: externalRequests.length,
      pageErrorCount: pageErrors.length,
      chapterResultsDigest,
      summarySha256: sha256File(summaryPath),
      chapterLogSha256: sha256File(chapterLogPath),
      determinismLogSha256: sha256File(determinismLogPath)
    },
    interpretation: 'Gold V21 native 79-chapter harness measurement. Every chapter from the pinned long corpus was measured independently; this is not a single continuous-document pagination baseline and must not be represented as one.'
  };
  const manifestPath = path.join(artifactDir, 'legacy-gold-v21-chapter-baseline.json');
  fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + String.fromCharCode(10), 'utf8');
  process.stdout.write('Gold V21 79-chapter profile measurement captured: ' + status + String.fromCharCode(10));
  process.stdout.write('Manifest: ' + relative(manifestPath) + String.fromCharCode(10));
}

try { main(); }
catch (error) { process.stderr.write(String(error && (error.stack || error)) + String.fromCharCode(10)); process.exitCode = 1; }
