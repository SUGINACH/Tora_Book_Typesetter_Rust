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
  cliSha256: '0eeb4d0af360ae4f2264abfbca904c91e0ea189e72870bd3ac6981ebd0839594',
  fixtureSha256: '1aec6688976243118051068752d72acc85b8830fdc0a0be96bd133c2b018276c',
  fontsCssSha256: 'eec01e2b2a65ab67ad959625dd7b3640dceeb09a5cee8b7677787a1e89acd229'
};
const reportRelative = 'artifacts/rust-legacy-smoke-report.json';
const summaryRelative = 'artifacts/rust-legacy-smoke-summary.md';

function sha256File(file) {
  return crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex');
}
function assert(condition, message) {
  if (!condition) throw new Error('Legacy runner verification failed: ' + message);
}

function main() {
  assert(sourceCommit === expected.sourceCommit,
    'LEGACY_SOURCE_COMMIT must equal the pinned Orange commit ' + expected.sourceCommit);
  assert(fs.existsSync(legacyRoot), 'pinned legacy checkout is missing: ' + legacyRoot);

  const actualCommit = execFileSync('git', ['rev-parse', 'HEAD'], {
    cwd: legacyRoot, encoding: 'utf8'
  }).trim();
  assert(actualCommit === expected.sourceCommit,
    'legacy checkout HEAD mismatch: expected ' + expected.sourceCommit + ', got ' + actualCommit);

  const cliPath = path.join(legacyRoot, 'tools', 'geometry-audit-cli.js');
  const fixturePath = path.join(legacyRoot, 'tests', 'fixtures', 'geometry-audit-smoke.txt');
  const fontsCssPath = path.join(legacyRoot, 'selfhost-fonts', 'google-fonts-selfhost.css');
  const packagePath = path.join(legacyRoot, 'node_modules', 'playwright', 'package.json');
  assert(sha256File(cliPath) === expected.cliSha256, 'pinned CLI content hash changed');
  assert(sha256File(fixturePath) === expected.fixtureSha256, 'pinned smoke fixture content hash changed');
  assert(sha256File(fontsCssPath) === expected.fontsCssSha256, 'self-hosted font CSS hash changed');
  assert(fs.existsSync(packagePath), 'Playwright is not installed; run npm ci in the pinned legacy checkout');
  const playwrightVersion = JSON.parse(fs.readFileSync(packagePath, 'utf8')).version;
  assert(playwrightVersion === expected.playwrightVersion,
    'Playwright version mismatch: expected ' + expected.playwrightVersion + ', got ' + playwrightVersion);

  fs.mkdirSync(path.dirname(path.join(legacyRoot, reportRelative)), { recursive: true });
  const args = [
    cliPath,
    '--input', 'tests/fixtures/geometry-audit-smoke.txt',
    '--report', reportRelative,
    '--summary', summaryRelative,
    '--fail-on', 'none',
    '--repeat-check',
    '--full-model'
  ];
  const child = spawnSync(process.execPath, args, {
    cwd: legacyRoot,
    encoding: 'utf8',
    stdio: 'inherit',
    env: {
      ...process.env,
      GITHUB_SHA: expected.sourceCommit,
      GITHUB_REF_NAME: 'legacy-orange-v23'
    }
  });
  assert(child.error == null, 'could not start pinned geometry audit CLI: ' + (child.error || 'unknown'));
  assert(child.status === 0, 'pinned geometry audit CLI exited with status ' + child.status);

  const reportPath = path.join(legacyRoot, reportRelative);
  const summaryPath = path.join(legacyRoot, summaryRelative);
  assert(fs.existsSync(reportPath), 'geometry audit JSON report was not written');
  assert(fs.existsSync(summaryPath), 'geometry audit Markdown summary was not written');
  const report = JSON.parse(fs.readFileSync(reportPath, 'utf8'));

  assert(report.run?.gitCommit === expected.sourceCommit, 'report does not identify the pinned legacy source commit');
  assert(report.run?.playwrightVersion === expected.playwrightVersion, 'report Playwright version differs from the pin');
  assert(report.run?.browserVersion === expected.chromiumVersion,
    'Chromium version mismatch: expected ' + expected.chromiumVersion + ', got ' + report.run?.browserVersion);
  assert(Array.isArray(report.run?.externalRequests) && report.run.externalRequests.length === 0,
    'browser attempted external network requests');
  assert(report.run?.fontsCssSha256 === expected.fontsCssSha256, 'report font CSS fingerprint differs from the pin');
  assert(report.run?.determinismCheck?.identical === true, 'repeat layout signatures are not identical');
  assert(Number.isInteger(report.pageCount) && report.pageCount > 0, 'report has no rendered pages');
  assert(Array.isArray(report.geometry?.layoutDecisionRecords) && report.geometry.layoutDecisionRecords.length > 0,
    'report has no serialized layout decision ledger');
  assert(report.geometry?.layoutDecisionRecordAudit?.recordCountMatchesDom === true,
    'decision ledger count does not match the rendered DOM');
  assert(report.geometry?.textConservationAudit?.available === true,
    'source-to-rendered text conservation audit is unavailable');
  assert(report.geometry?.fontsStatus === 'loaded', 'font status is not loaded');
  const fontPreflight = report.geometry?.fontPreflight;
  assert(Number.isInteger(fontPreflight?.faceCount) && fontPreflight.faceCount > 0,
    'font preflight did not enumerate any font faces');
  assert(fontPreflight.loaded === fontPreflight.faceCount && fontPreflight.failed === 0 &&
    Array.isArray(fontPreflight.failures) && fontPreflight.failures.length === 0,
    'not all enumerated font faces loaded successfully');
  assert(report.runtime?.pageErrors?.length === 0, 'browser page errors were reported');

  const artifactDir = path.join(root, 'artifacts');
  fs.mkdirSync(artifactDir, { recursive: true });
  const manifest = {
    schemaVersion: 1,
    runner: 'tools/legacy-runner/run.cjs',
    targetRustCommit: rustCommit,
    legacy: {
      repository: 'SUGINACH/Tora_Book_Typesetter_Orange',
      branchProfile: 'legacy-orange-v23',
      commit: actualCommit,
      cliSha256: sha256File(cliPath),
      fixturePath: 'tests/fixtures/geometry-audit-smoke.txt',
      fixtureSha256: sha256File(fixturePath),
      fontsCssPath: 'selfhost-fonts/google-fonts-selfhost.css',
      fontsCssSha256: sha256File(fontsCssPath)
    },
    environment: {
      node: process.version,
      playwright: playwrightVersion,
      chromium: report.run.browserVersion,
      viewport: report.run.viewport
    },
    result: {
      pageCount: report.pageCount,
      issueSummary: report.audit?.summary || null,
      decisionRecordCount: report.geometry.layoutDecisionRecords.length,
      decisionLedgerMatchesDom: report.geometry.layoutDecisionRecordAudit.recordCountMatchesDom,
      fontPreflight,
      textConservation: report.geometry.textConservationAudit,
      externalRequestCount: report.run.externalRequests.length,
      deterministicRepeat: report.run.determinismCheck.identical,
      reportSha256: sha256File(reportPath),
      summarySha256: sha256File(summaryPath)
    }
  };
  const manifestPath = path.join(artifactDir, 'legacy-runner-manifest.json');
  fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n', 'utf8');
  process.stdout.write('Legacy runner verification: PASS\n');
  process.stdout.write('Manifest: ' + path.relative(root, manifestPath) + '\n');
}

try {
  main();
} catch (error) {
  process.stderr.write(String(error && (error.stack || error)) + '\n');
  process.exitCode = 1;
}
