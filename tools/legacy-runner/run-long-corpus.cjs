'use strict';

const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const { spawnSync, execFileSync } = require('node:child_process');

const root = path.resolve(__dirname, '../..');
const legacyRoot = path.join(root, 'legacy', 'orange');
const sourceCommit = process.env.LEGACY_SOURCE_COMMIT;
const rustCommit = process.env.RUST_TARGET_COMMIT || process.env.GITHUB_SHA || null;
const expected = {
  sourceCommit: '0b0ee279464a3db14672cd3f4051275aba5985da',
  playwrightVersion: '1.55.0',
  chromiumVersion: '140.0.7339.16',
  inputSha256: '5e73b913f9843fce695d0e4073c176961b8dbc2fa87e8e4c3d99051a826ce0e4',
  settingsSha256: '5c78a49ca2b205f0a846e6f87a1058b4a61595967b472bda65ac7412f13f5b3f',
  fontsCssSha256: 'eec01e2b2a65ab67ad959625dd7b3640dceeb09a5cee8b7677787a1e89acd229',
  expectedBaselineSha256: '9a5692bd109b6ba8d572e83daf71fc3663c32eff3210cc330fb17e60f1c037df'
};
const reportRelative = 'artifacts/rust-legacy-long-report.json';
const summaryRelative = 'artifacts/rust-legacy-long-summary.md';

function sha256File(file) {
  return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
}
function assert(condition, message) {
  if (!condition) throw new Error('Legacy long-corpus verification failed: ' + message);
}

function main() {
  assert(sourceCommit === expected.sourceCommit, 'LEGACY_SOURCE_COMMIT does not match the Orange pin');
  const actualCommit = execFileSync('git', ['rev-parse', 'HEAD'], {
    cwd: legacyRoot, encoding: 'utf8'
  }).trim();
  assert(actualCommit === expected.sourceCommit, 'legacy checkout HEAD differs from the pinned commit');

  const inputPath = path.join(legacyRoot, 'very_long_test_text.txt');
  const settingsPath = path.join(legacyRoot, 'default_settings.json');
  const fontsCssPath = path.join(legacyRoot, 'selfhost-fonts', 'google-fonts-selfhost.css');
  const baselinePath = path.join(legacyRoot, 'expected_baseline.json');
  const cliPath = path.join(legacyRoot, 'tools', 'geometry-audit-cli.js');
  const playwrightPath = path.join(legacyRoot, 'node_modules', 'playwright', 'package.json');

  assert(sha256File(inputPath) === expected.inputSha256, 'long corpus SHA-256 mismatch');
  assert(sha256File(settingsPath) === expected.settingsSha256, 'default settings SHA-256 mismatch');
  assert(sha256File(fontsCssPath) === expected.fontsCssSha256, 'self-hosted font CSS SHA-256 mismatch');
  assert(sha256File(baselinePath) === expected.expectedBaselineSha256, 'historical baseline file SHA-256 mismatch');
  const playwrightVersion = JSON.parse(fs.readFileSync(playwrightPath, 'utf8')).version;
  assert(playwrightVersion === expected.playwrightVersion, 'Playwright version mismatch');

  fs.mkdirSync(path.dirname(path.join(legacyRoot, reportRelative)), { recursive: true });
  const child = spawnSync(process.execPath, [
    cliPath,
    '--input', 'very_long_test_text.txt',
    '--settings', 'default_settings.json',
    '--timeout-ms', '900000',
    '--report', reportRelative,
    '--summary', summaryRelative,
    '--fail-on', 'none',
    '--repeat-check',
    '--full-model'
  ], {
    cwd: legacyRoot,
    encoding: 'utf8',
    stdio: 'inherit',
    env: {
      ...process.env,
      GITHUB_SHA: expected.sourceCommit,
      GITHUB_REF_NAME: 'legacy-orange-v23'
    }
  });
  assert(child.error == null, 'could not start the pinned long-corpus CLI');
  assert(child.status === 0, 'long-corpus CLI exited with status ' + child.status);

  const reportPath = path.join(legacyRoot, reportRelative);
  const summaryPath = path.join(legacyRoot, summaryRelative);
  assert(fs.existsSync(reportPath) && fs.existsSync(summaryPath), 'long-corpus report/summary missing');
  const report = JSON.parse(fs.readFileSync(reportPath, 'utf8'));
  const historical = JSON.parse(fs.readFileSync(baselinePath, 'utf8')).default_config;

  assert(report.run?.settingsSource === 'default_settings.json',
    'runner did not explicitly use the pinned default settings file');
  const pinnedSettings = JSON.parse(fs.readFileSync(settingsPath, 'utf8'));
  assert(JSON.stringify(report.run?.settings) === JSON.stringify(pinnedSettings),
    'effective settings differ from the pinned default_settings.json bytes');
  assert(report.run?.timeoutMs === 900000, 'full-corpus render timeout was not set to 900000 ms');
  assert(report.run?.gitCommit === expected.sourceCommit, 'report source commit mismatch');
  assert(report.run?.playwrightVersion === expected.playwrightVersion, 'report Playwright mismatch');
  assert(report.run?.browserVersion === expected.chromiumVersion, 'report Chromium mismatch');
  assert(report.run?.inputSha256 === expected.inputSha256, 'report input fingerprint mismatch');
  assert(report.run?.fontsCssSha256 === expected.fontsCssSha256, 'report font CSS fingerprint mismatch');
  assert(Array.isArray(report.run?.externalRequests) && report.run.externalRequests.length === 0,
    'browser attempted external network requests');
  assert(report.run?.determinismCheck?.identical === true, 'repeat-run layout signatures differ');
  assert(Number.isInteger(report.pageCount) && report.pageCount > 0, 'no pages rendered');

  const fontPreflight = report.geometry?.fontPreflight;
  assert(Number.isInteger(fontPreflight?.faceCount) && fontPreflight.faceCount > 0,
    'font preflight did not enumerate faces');
  assert(fontPreflight.loaded === fontPreflight.faceCount && fontPreflight.failed === 0 &&
    Array.isArray(fontPreflight.failures) && fontPreflight.failures.length === 0,
    'not all font faces loaded');
  assert(report.geometry?.layoutDecisionRecordAudit?.recordCountMatchesDom === true,
    'decision ledger does not match DOM blocks');
  assert(report.geometry?.textConservationAudit?.available === true,
    'text conservation audit unavailable');
  assert(report.geometry.textConservationAudit.missingSourceParagraphCount === 0 &&
    report.geometry.textConservationAudit.textMismatchCount === 0 &&
    report.geometry.textConservationAudit.headingMissingCount === 0 &&
    report.geometry.textConservationAudit.headingMismatchCount === 0,
    'text conservation reported missing/mismatched source content');
  assert(report.runtime?.pageErrors?.length === 0, 'browser page errors were reported');

  const actual = {
    pages: report.pageCount,
    issues: report.audit?.summary?.total ?? null,
    high: report.audit?.summary?.high ?? null,
    medium: report.audit?.summary?.medium ?? null,
    low: report.audit?.summary?.low ?? null,
    ledger: report.geometry.layoutDecisionRecords.length
  };
  const expectedMetrics = {
    pages: historical.pages,
    issues: historical.issues,
    high: historical.high,
    medium: historical.medium,
    low: historical.low,
    ledger: Number(String(historical.ledger).split('/')[0])
  };
  const metricsMatch = Object.keys(expectedMetrics).every(key => actual[key] === expectedMetrics[key]);

  const artifactDir = path.join(root, 'artifacts');
  fs.mkdirSync(artifactDir, { recursive: true });
  const manifest = {
    schemaVersion: 1,
    targetRustCommit: rustCommit,
    legacy: {
      repository: 'SUGINACH/Tora_Book_Typesetter_Orange',
      profile: 'legacy-orange-v23',
      commit: actualCommit,
      inputPath: 'very_long_test_text.txt',
      inputSha256: sha256File(inputPath),
      settingsPath: 'default_settings.json',
      settingsSha256: sha256File(settingsPath),
      fontsCssPath: 'selfhost-fonts/google-fonts-selfhost.css',
      fontsCssSha256: sha256File(fontsCssPath),
      historicalBaselinePath: 'expected_baseline.json',
      historicalBaselineSha256: sha256File(baselinePath)
    },
    environment: {
      node: process.version,
      playwright: playwrightVersion,
      chromium: report.run.browserVersion,
      viewport: report.run.viewport
    },
    measured: {
      ...actual,
      decisionLedgerMatchesDom: report.geometry.layoutDecisionRecordAudit.recordCountMatchesDom,
      fontPreflight,
      textConservation: report.geometry.textConservationAudit,
      externalRequestCount: report.run.externalRequests.length,
      deterministicRepeat: report.run.determinismCheck.identical,
      browserPageErrors: report.runtime.pageErrors.length,
      browserConsoleErrors: report.runtime.consoleErrors?.length ?? null,
      renderMilliseconds: report.run.renderMs ?? null,
      reportSha256: sha256File(reportPath),
      summarySha256: sha256File(summaryPath)
    },
    historicalComparison: {
      expected: expectedMetrics,
      actual,
      matchesHistoricalMetrics: metricsMatch,
      interpretation: metricsMatch
        ? 'Metrics match the historical expected_baseline.json under the current pinned environment.'
        : 'Metrics differ from the historical expected_baseline.json. This is recorded as a new environment-specific baseline, not silently normalized; compare browser/font/settings fingerprints before attributing a regression.'
    }
  };
  const manifestPath = path.join(artifactDir, 'legacy-long-corpus-manifest.json');
  fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n', 'utf8');
  process.stdout.write('Legacy long-corpus integrity and determinism: PASS\n');
  process.stdout.write('Historical metric comparison: ' + (metricsMatch ? 'MATCH' : 'DIFFERENT — see manifest') + '\n');
}

try {
  main();
} catch (error) {
  process.stderr.write(String(error && (error.stack || error)) + '\n');
  process.exitCode = 1;
}
