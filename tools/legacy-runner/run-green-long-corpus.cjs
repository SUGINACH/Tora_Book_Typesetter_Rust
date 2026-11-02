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
  styleCssSha256: 'd6fcc447e5bc57a4c62ac4d68bd0f0b939905390cafa8e5af95b82770ab7ea93',
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

function buildOfflineAuditCli(sourcePath) {
  const source = fs.readFileSync(sourcePath, 'utf8');
  const declaration = "const pageErrors = [], consoleErrors = [], started = Date.now();";
  const replacement = "const pageErrors = [], consoleErrors = [], externalRequests = [], started = Date.now();";
  assert(source.includes(declaration), 'pinned CLI instrumentation anchor changed');
  let patched = source.replace(declaration, replacement);

  const pageHook = "        page.on('pageerror', e => pageErrors.push(String(e && (e.stack || e))));";
  const routeHook = [
    "        const localOrigin = new URL(baseUrl).origin;",
    "        await page.route('**/*', async route => {",
    "            const requestUrl = new URL(route.request().url());",
    "            if (requestUrl.origin === localOrigin ||",
    "                ['data:', 'blob:', 'about:'].includes(requestUrl.protocol)) {",
    "                await route.continue();",
    "            } else {",
    "                externalRequests.push(requestUrl.href);",
    "                if (process.env.GREEN_EXTERNAL_REQUESTS_PATH) require('node:fs').appendFileSync(process.env.GREEN_EXTERNAL_REQUESTS_PATH, requestUrl.href + String.fromCharCode(10));",
    "                await route.abort('blockedbyclient');",
    "            }",
    "        });",
    "        page.on('pageerror', e => pageErrors.push(String(e && (e.stack || e))));"
  ].join('\n');
  assert(patched.includes(pageHook), 'pinned CLI page-hook anchor changed');
  patched = patched.replace(pageHook, routeHook);

  const reportHook = "browser:'Chromium headless' },";
  assert(patched.includes(reportHook), 'pinned CLI report anchor changed');
  patched = patched.replace(reportHook, "externalRequests, browser:'Chromium headless' },");
  const outputPath = sourcePath.replace('geometry-audit-cli.js', 'geometry-audit-cli.offline.cjs');
  fs.writeFileSync(outputPath, patched, 'utf8');
  return outputPath;
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

  const browser = spawnSync(process.execPath, ['-e',
    "const { chromium } = require('playwright'); (async()=>{const b=await chromium.launch({headless:true}); process.stdout.write(b.version()); await b.close();})().catch(e=>{console.error(e);process.exit(1);});"
  ], { cwd: legacyRoot, encoding: 'utf8' });
  assert(browser.status === 0, 'could not query the pinned Chromium build');
  assert(browser.stdout.trim() === expected.chromiumVersion,
    'Chromium version mismatch: expected ' + expected.chromiumVersion + ', got ' + browser.stdout.trim());

  const offlineFonts = prepareOfflineFonts();
  const offlineCliPath = buildOfflineAuditCli(cliPath);
  const reportRelative = 'artifacts/rust-green-long-report.json';
  const summaryRelative = 'artifacts/rust-green-long-summary.md';
  const reportPath = path.join(legacyRoot, reportRelative);
  const summaryPath = path.join(legacyRoot, summaryRelative);
  fs.mkdirSync(path.dirname(reportPath), { recursive: true });
  const externalRequestsPath = path.join(root, 'artifacts', 'green-external-requests.txt');
  const failureLogPath = path.join(root, 'artifacts', 'legacy-green-layout-failure.log');
  fs.mkdirSync(path.dirname(externalRequestsPath), { recursive: true });
  const args = [
    offlineCliPath,
    '--input', expected.inputPath,
    '--settings', expected.settingsPath,
    '--timeout-ms', '900000',
    '--report', reportRelative,
    '--summary', summaryRelative,
    '--fail-on', 'none',
    '--repeat-check',
    '--full-model',
    '--step-audit-disabled'
  ];
  const runAttempt = attempt => {
    fs.writeFileSync(externalRequestsPath, '', 'utf8');
    const result = spawnSync(process.execPath, args, {
      cwd: legacyRoot,
      encoding: 'utf8',
      stdio: 'pipe',
      maxBuffer: 32 * 1024 * 1024,
      env: {
        ...process.env,
        GITHUB_SHA: expected.sourceCommit,
        GITHUB_REF_NAME: 'legacy-green-v23',
        GREEN_EXTERNAL_REQUESTS_PATH: externalRequestsPath
      }
    });
    if (result.stdout) process.stdout.write(result.stdout);
    if (result.stderr) process.stderr.write(result.stderr);
    assert(result.error == null, 'could not start Green geometry audit CLI on attempt ' + attempt);
    const externalRequests = fs.readFileSync(externalRequestsPath, 'utf8').split(/\\r?\\n/).filter(Boolean);
    return { ...result, externalRequests };
  };
  const first = runAttempt(1);
  if (first.status !== 0) {
    const failurePrefix = 'Layout infeasible without violating widow/orphan rules: no content progress across 2 pages at tokenIndex=';
    const locateFailure = output => {
      const markerIndex = output.indexOf(failurePrefix);
      if (markerIndex < 0) return null;
      const detail = output.slice(markerIndex + failurePrefix.length).split(String.fromCharCode(10))[0].trimEnd();
      const pendingMarker = ', pending=';
      const comma = detail.indexOf(pendingMarker);
      const pipe = detail.indexOf('|', comma + pendingMarker.length);
      if (comma < 1 || pipe < 0) return null;
      const tokenIndex = Number(detail.slice(0, comma));
      const pendingCount = Number(detail.slice(comma + pendingMarker.length, pipe));
      if (!Number.isInteger(tokenIndex) || !Number.isInteger(pendingCount)) return null;
      return { tokenIndex, pendingCount, pendingTextPreview: detail.slice(pipe + 1, pipe + 181) };
    };
    const firstText = String(first.stderr || '') + String.fromCharCode(10) + String(first.stdout || '');
    const firstSignature = locateFailure(firstText);
    if (firstSignature) {
      const second = runAttempt(2);
      const secondText = String(second.stderr || '') + String.fromCharCode(10) + String(second.stdout || '');
      const secondSignature = locateFailure(secondText);
      assert(second.status !== 0 && secondSignature, 'second Green attempt did not reproduce the same layout-infeasible class');
      assert(JSON.stringify(firstSignature) === JSON.stringify(secondSignature), 'Green layout-infeasible signature was not deterministic');
      assert(first.externalRequests.length === 0 && second.externalRequests.length === 0, 'Green failure attempts tried external network requests');
      fs.writeFileSync(failureLogPath, [
        '=== attempt 1 stdout ===', first.stdout || '',
        '=== attempt 1 stderr ===', first.stderr || '',
        '=== attempt 2 stdout ===', second.stdout || '',
        '=== attempt 2 stderr ===', second.stderr || ''
      ].join('\\n'), 'utf8');
      const failureManifest = {
        schemaVersion: 1,
        status: 'blocked-layout-infeasible',
        fullCorpusComplete: false,
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
          offlineCliSha256: sha256File(offlineCliPath),
          fontCss: expected.fontCss,
          offlineFonts
        },
        environment: {
          node: process.version,
          playwright: playwrightVersion,
          chromium: browser.stdout.trim(),
          network: 'offline-enforced by an isolated Playwright route guard; only local renderer origin, data:, blob: and about: are allowed',
          viewport: { width: 1440, height: 1000, deviceScaleFactor: 1 },
          timeoutMs: 900000
        },
        failure: {
          kind: 'layout-infeasible-widow-orphan-no-progress',
          signature: firstSignature,
          attempts: 2,
          deterministicSignature: true,
          exitCodes: [first.status, second.status],
          externalRequests: []
        },
        failureLogSha256: sha256File(failureLogPath),
        interpretation: 'Pinned Green V23 full-corpus attempt is blocked by a deterministic legacy layout-infeasible condition at the recorded token. This is a measured failure fingerprint, not a complete full-corpus baseline, not a Rust parity result and not a passing profile gate.'
      };
      const artifactDir = path.join(root, 'artifacts');
      fs.mkdirSync(artifactDir, { recursive: true });
      const manifestPath = path.join(artifactDir, 'legacy-green-long-corpus-manifest.json');
      fs.writeFileSync(manifestPath, JSON.stringify(failureManifest, null, 2) + '\\n', 'utf8');
      process.stdout.write('Green V23 full-corpus attempt: deterministic layout-infeasible result captured; profile baseline remains blocked.\\n');
      process.stdout.write('Failure manifest: ' + relative(manifestPath) + '\\n');
      return;
    }
    throw new Error('Green geometry audit failed for an unclassified reason; see captured output. Exit status: ' + first.status);
  }
  assert(first.externalRequests.length === 0, 'browser attempted external network requests');
  assert(fs.existsSync(reportPath) && fs.existsSync(summaryPath), 'full-corpus report/summary missing');
  const report = JSON.parse(fs.readFileSync(reportPath, 'utf8'));
  assert(report.run?.gitCommit === expected.sourceCommit, 'report source commit mismatch');
  assert(report.run?.settingsSource === expected.settingsPath, 'report did not use pinned default settings');
  assert(JSON.stringify(report.run?.settings) === JSON.stringify(JSON.parse(fs.readFileSync(settingsPath, 'utf8'))),
    'effective settings differ from pinned default_settings.json');
  assert(report.run?.determinismCheck?.identical === true, 'repeat-render signatures are not identical');
  assert(Array.isArray(report.run?.externalRequests) && report.run.externalRequests.length === 0,
    'browser attempted external network requests');
  assert(report.run?.stepAudit?.enabled === false && report.run.stepAudit.disabledByRequest === true,
    'full-corpus run must explicitly disable per-step tracing');
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
  assert(textAudit.missingSourceParagraphCount === 0 && textAudit.textMismatchCount === 0 &&
    textAudit.renderedIdsWithoutSourceCount === 0 && textAudit.headingAudit?.missingCount === 0 &&
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
    externalRequestCount: report.run.externalRequests.length,
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
    status: 'complete',
    fullCorpusComplete: true,
    runner: 'tools/legacy-runner/run-green-long-corpus.cjs',
    targetRustCommit: process.env.RUST_TARGET_COMMIT || process.env.GITHUB_SHA || null,
    legacy: {
      repository: 'SUGINACH/Tora_Book_Typesetter_Green', profile: 'legacy-green-v23', commit: actualCommit,
      inputPath: expected.inputPath, inputSha256: sha256File(inputPath),
      settingsPath: expected.settingsPath, settingsSha256: sha256File(settingsPath),
      geometryAuditCliPath: expected.cliPath, geometryAuditCliSha256: sha256File(cliPath),
      offlineCliSha256: sha256File(offlineCliPath), fontCss: expected.fontCss, offlineFonts
    },
    environment: {
      node: process.version, playwright: playwrightVersion, chromium: browser.stdout.trim(),
      network: 'offline-enforced by an isolated Playwright route guard; only local renderer origin, data:, blob: and about: are allowed',
      viewport: report.run.viewport, timeoutMs: 900000
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

try { main(); }
catch (error) { process.stderr.write(String(error && (error.stack || error)) + '\n'); process.exitCode = 1; }
