# Legacy compatibility manifest schema

The compatibility manifest is a deterministic inventory of source records. It is not an implementation claim and does not copy source code into the Rust engine.

The top-level object contains `schemaVersion`, `generatedBy`, a sorted/pinned `sources` array (repository, commit, owningProfile), and the canonical `records` array. Each record's JSON key names below are the actual serialized field names emitted by `xtask compat-scan`.

## Record fields

| Field | Meaning |
|---|---|
| `compatId` | Stable identifier derived from profile, source path and record kind |
| `externalIdentifier` | Human-readable source/profile identifier |
| `repository` | Exact source repository in `owner/name` form |
| `commit` | Exact immutable Git commit used for the record |
| `owningProfile` | Isolated legacy profile identity |
| `legacyName` | Original basename of the source file |
| `sourcePath` | Repository-relative path |
| `sourceSha256` | SHA-256 of the exact source bytes |
| `kind` | File/record classification (engine, DOM, style, settings, data, font, asset, fixture, corpus, documentation, etc.) |
| `status` | Current evidence and restriction status |
| `parserOrConsumer` | Legacy consumer or intended Rust reader boundary |
| `assetDependencies` | Recognized local and external resource references |
| `unresolvedAssetDependencies` | Local references not found in the same profile inventory |
| `assetDependencyStatus` | `all-local-resolved`, `unresolved-local`, `contains-external`, or `no-references-detected` |
| `parityFixtures` | Exact profile/commit/path fixture references |
| `rustOwner` | Rust subsystem responsible for migration or resolution |
| `migrationRule` | Explicit migration boundary; not an assertion that code exists |
| `testIds` | Stable identifiers of required tests |
| `acceptanceCriteria` | Observable conditions required before a record can be accepted |
| `verificationLevel` | Strongest evidence actually completed for this record |
| `verificationEvidence` | Exact runner URL, runner commit and evidence role supporting a non-hash verification level |
| `notes` | Limitations, conflicts and manual-review requirements |

## Status semantics

- `verified-executable`: this exact source record was executed by a named pinned runner/fixture.
- `verified-data`: this exact data record was validated by a named check; a hash alone is insufficient.
- `historical-only`: preserved for traceability, not accepted as an executable implementation.
- `spec-only`: requirements/research material, not executable input.
- `conflict`: unresolved disagreement between sources or requirements.
- `blocked-license`: redistribution or bundling is prohibited until licensing review is complete.

## Dependency semantics

- `local:path/from/profile/root` is a normalized local path relative to the source repository root.
- `external:https://...` is an external URL discovered in a source reference.
- Resolution is profile-scoped. A file in one legacy profile cannot silently satisfy a reference in another.
- The extractor currently recognizes CSS `url()` and imports, HTML/SVG resource attributes, and path-like JavaScript/JSON strings. Computed paths and runtime-generated references remain a manual-review risk.
- `no-references-detected` is not proof that a record has no dependencies.

## Verification semantics

- `scanned-hash-only`: immutable source identity and file bytes were recorded; behavior was not executed.
- `offline-smoke-tested`: the exact profile was exercised by the isolated offline smoke runner.
- `offline-full-corpus-tested`: the exact profile passed the complete pinned corpus.
- `native-parity-tested`: output was compared against a separately executed native reference.
- Font-specific validation and license clearance must be recorded separately; successful loading or shaping does not imply redistribution permission.

## Determinism

Records are sorted by source profile and source path. Dependency arrays are deduplicated and sorted. The generator emits stable JSON and is run twice in CI; the generated files must be byte-identical. Every update must preserve the source commit and SHA-256 evidence.
