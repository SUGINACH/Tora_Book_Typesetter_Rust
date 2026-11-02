'use strict';

const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const { execFileSync, spawnSync } = require('node:child_process');

const root = path.resolve(__dirname, '../..');
const legacyRoot = path.join(root, 'legacy', 'green');
const expected = {
  sourceCommit: 'e5cde5f194d0d9e9b20fdaa2a742f9bf6e8aec53',
  inputPath: 'text_files_for_typesetter_tests/very_long_test_text.txt',
  inputSha256: 'fdaf3fe024ffff3f547c0fe8ce27c2da287715c911cbe2000338d19f1438c282',
  settingsPath: 'default_settings.json',
  settingsSha256: 'cc03174d6d8df360813ee34a7718e2745364e5a5c8db21774ffa9c09dd8fb609',
  cliPath: 'tools/geometry-audit-cli.js',
  cliSha256: '5cea7320286de51c09397f6bd492eeadfa8d8f0678ff8b99ba25684cc8f98971',
  fontCss: [
    { path: 'fonts-def.css', sha256: 'e8739f6aad0cff6aadae6faec39d4aecada85b2303d7548b854a3d7d77bfc48f' },
    { path: 'fonts-extended.css', sha256: '1d038492d7ded85b5967f3f5bb83b56eb5b2c3b900c0e72ce70adf31e62da277' }
  ],
  playwrightVersion: '1.55.0',
  chromiumVersion: '140.0.7339.16'
};

function sha256File(file) {
  return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
}
function assert(condition, message) {
  if (!condition) throw new Error('Green V23 baseline verification failed: ' + message);
}
function relative(file) {
  return path.relative(root, file).split(path.sep).join('/');
}

function main() {
  assert(process.env.LEGACY_SOURCE_COMMIT === expected.sourceCommit,
    'LEGACY_SOURCE_COMMIT does not match the exact Green V23 pin');
  assert(fs.existsSync(legacyRoot), 'pinned Green checkout is missing');
  const actualCommit = execFileSync('git', ['rev-parse', 'HEAD'], {
    cwd: legacyRoot, encoding: 'utf8'
  }).trim();
  assert(actualCommit === expected.sourceCommit, 'Green checkout HEAD differs from the pin');

  const inputPath = path.join(legacyRoot, expected.inputPath);
  const settingsPath = path.join(legacyRoot, expected.settingsPath);
  const cliPath = path.join(legacyRoot, expected.cliPath);
  assert(sha256File(inputPath) === expected.inputSha256, 'long-corpus SHA-256 mismatch');
  assert(sha256File(settingsPath) === expected.settingsSha256, 'default-settings SHA-256 mismatch');
  assert(sha256File(cliPath) === expected.cliSha256, 'geometry audit CLI SHA-256 mismatch');
  for (const font of expected.fontCss) {
    assert(sha256File(path.join(legacyRoot, font.path)) === font.sha256,
      'font CSS SHA-256 mismatch: ' + font.path);
  }

  const playwrightPath = path.join(legacyRoot, 'node_modules', 'playwright', 'package.json');
  assert(fs.existsSync(playwrightPath), 'Playwright is not installed');
  const playwrightVersion = JSON.parse(fs.readFileSync(playwrightPath, 'utf8')).version;
  assert(playwrightVersion === expected.playwrightVersion,
    'Playwright version mismatch: expected ' + expected.playwrightVersion + ', got ' + playwrightVersion);

  const { chromium } = require(path.join(legacyRoot, 'node_modules', 'playwright'));
  const browser = spawnSync(process.execPath, ['-e',
    "const { chromium } = require('playwright'); (async()=>{const b=await chromium.launch({headless:true}); process.stdout.write(b.version()); await b.close();})().catch(e=>{console.error(e);process.exit(1);});"
  ], { cwd: legacyRoot, encoding: 'utf8' });
  assert(browser.status === 0, 'could not query the pinned Chromium build');
  assert(browser.stdout.trim() === expected.chromiumVersion,
    'Chromium version mismatch: expected ' + expected.chromiumVersion + ', got ' + browser.stdout.trim());

  const reportRelative = 'artifacts/rust-green-long-report.json';
  const summaryRelative = 'artifacts/rust-green-long-summary.md';
  const reportPath = path.join(legacyRoot, reportRelative);
  const summaryPath = path.join(legacyRoot, summaryRelative);
  fs.mkdirSync(path.dirname(reportPath), { recursive: true });
  const child = spawnSync(process.execPath, [
    cliPath,
    '--input', expected.inputPath,
    '--settings', expected.settingsPath,
    '--timeout-ms', '900000',
    '--report', reportRelative,
    '--summary', summaryRelative,
    '--fail-on', 'none',
    '--repeat-check',
    '--full-model',
    '--step-audit-disabled'
  ], {
    cwd: legacyRoot,
    encoding: 'utf8',
    stdio: 'inherit',
    env: {
      ...process.env,
      GITHUB_SHA: expected.sourceCommit,
      GITHUB_REF_NAME: 'legacy-green-v23'
    }
  });
  assert(child.error == null, 'could not start Green geometry audit CLI');
  assert(child.status === 0, 'Green geometry audit CLI exited with status ' + child.status);
  assert(fs.existsSync(reportPath) && fs.existsSync(summaryPath), 'full-corpus report/summary missing');

  const report = JSON.parse(fs.readFileSync(reportPath, 'utf8'));
  assert(report.run?.gitCommit === expected.sourceCommit, 'report source commit mismatch');
  assert(report.run?.settingsSource === expected.settingsPath, 'report did not use pinned default settings');
  assert(JSON.stringify(report.run?.settings) === JSON.stringify(JSON.parse(fs.readFileSync(settingsPath, 'utf8'))),
    'effective settings differ from pinned default_settings.json');
  assert(report.run?.determinismCheck?.identical === true, 'repeat-render signatures are not identical');
  assert(Array.isArray(report.run?.externalRequests) && report.run.externalRequests.length === 0,
    'browser attempted external network requests');
  assert(Number.isInteger(report.pageCount) && report.pageCount > 0, 'no pages rendered');

  const fontPreflight = report.geometry?.fontPreflight;
  assert(Number.isInteger(fontPreflight?.faceCount) && fontPreflight.faceCount > 0,
    'font preflight did not enumerate font faces');
  assert(fontPreflight.loaded === fontPreflight.faceCount && fontPreflight.failed === 0 &&
    Array.isArray(fontPreflight.failures) && fontPreflight.failures.length === 0,
    'not all enumerated font faces loaded');
  assert(report.geometry?.layoutDecisionRecordAudit?.recordCountMatchesDom === true,
    'decision ledger does not match DOM blocks');
  const textAudit = report.geometry?.textConservationAudit;
  assert(textAudit?.available === true, 'source-to-rendered text conservation audit unavailable');
  assert(textAudit.missingSourceParagraphCount === 0 &&
    textAudit.textMismatchCount === 0 &&
    textAudit.renderedIdsWithoutSourceCount === 0 &&
    textAudit.headingAudit?.missingCount === 0 &&
    textAudit.headingAudit?.mismatchCount === 0,
    'text conservation reported missing/mismatched paragraphs or headings');
  assert(report.runtime?.pageErrors?.length === 0, 'browser page errors were reported');

  const measured = {
    pages: report.pageCount,
    issues: report.audit?.summary?.total ?? null,
    high: report.audit?.summary?.high ?? null,
    medium: report.audit?.summary?.medium ?? null,
    low: report.audit?.summary?.low ?? null,
    decisionLedgerRecords: report.geometry.layoutDecisionRecords?.length ?? null,
    decisionLedgerMatchesDom: report.geometry.layoutDecisionRecordAudit.recordCountMatchesDom,
    sourceParagraphs: textAudit.sourceParagraphCount ?? null,
    missingSourceParagraphs: textAudit.missingSourceParagraphCount,
    textMismatches: textAudit.textMismatchCount,
    renderedIdsWithoutSource: textAudit.renderedIdsWithoutSourceCount,
    headingAudit: textAudit.headingAudit ?? null,
    fontPreflight,
    externalRequests: report.run.externalRequests.length,
    deterministicRepeat: report.run.determinismCheck.identical,
    browserPageErrors: report.runtime.pageErrors.length,
    browserConsoleErrors: report.runtime.consoleErrors?.length ?? null,
    renderMilliseconds: report.run.renderMs ?? null,
    viewport: report.run.viewport,
    reportSha256: sha256File(reportPath),
    summarySha256: sha256File(summaryPath)
  };

  const manifest = {
    schemaVersion: 1,
    runner: 'tools/legacy-runner/run-green-long-corpus.cjs',
    targetRustCommit: process.env.RUST_TARGET_COMMIT || process.env.GITHUB_SHA || null,
    legacy: {
      repository: 'SUGINACH/Tora_Book_Typesetter_Green',
      profile: 'legacy-green-v23',
      commit: actualCommit,
      inputPath: expected.inputPath,
      inputSha256: sha256File(inputPath),
      settingsPath: expected.settingsPath,
      settingsSha256: sha256File(settingsPath),
      geometryAuditCliPath: expected.cliPath,
      geometryAuditCliSha256: sha256File(cliPath),
      fontCss: expected.fontCss
    },
    environment: {
      node: process.version,
      playwright: playwrightVersion,
      chromium: browser.stdout.trim(),
      network: 'offline-enforced by pinned geometry-audit CLI',
      viewport: report.run.viewport,
      timeoutMs: 900000
    },
    measured,
    interpretation: 'Independent measured Green V23 full-corpus baseline. Do not compare or merge its values with Orange V23, Gold V21, Yellow V10 or IMUD V20.x.'
  };
  const artifactDir = path.join(root, 'artifacts');
  fs.mkdirSync(artifactDir, { recursive: true });
  const manifestPath = path.join(artifactDir, 'legacy-green-long-corpus-manifest.json');
  fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n', 'utf8');
  process.stdout.write('Green V23 full-corpus baseline verification: PASS\n');
  process.stdout.write('Manifest: ' + relative(manifestPath) + '\n');
}

try {
  main();
} catch (error) {
  process.stderr.write(String(error && (error.stack || error)) + '\n');
  process.exitCode = 1;
}
