# קובץ מעבר — Tora Book Typesetter Rust

עודכן: 2026-11-02 (לאחר CI ירוק וריצות manifest/full-corpus פעילות)  
מאגר: [SUGINACH/Tora_Book_Typesetter_Rust](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust)  
ענף: `main`  
נקודת בסיס לפני עדכון המעבר: `028319268d56eb2492ed36a16d861f02c2bcfcc7`

## מטרת העבודה

מנוע עימוד תורני ב-Rust, דטרמיניסטי ושקוף, עם מודל גאומטרי מדויק, הפרדה בין semantic/layout/rendering, decision ledger בר-ביקורת וייצוא שאינו תלוי במדידות WebView. תוכנית 100 המשימות היא מסגרת הביצוע המחייבת. בכל הודעת המשך יש להשלים יחידת עבודה אחת מוגדרת, להריץ את הבדיקות הרלוונטיות, לדחוף commit ולעדכן את שלושת מסמכי המעקב.

## מצב כולל

- שלב א (משימות 1–10) פעיל ועדיין פתוח.
- משימות 1–2 הושלמו והוכחו ב-[CI 41347908551](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41347908551).
- אין להתחיל שלב ב עד סגירת משימות 3–10 ושערי הקבלה שלהן.
- אין לטעון שקיים מנוע עימוד עובד: עדיין אין מימוש production של parser, shaping, pagination או renderer.
- V21 ו-V23 נשארים מבודדים לחלוטין. אין למזג קוד, fixtures, baselines או החלטות אלגוריתמיות ביניהם.

## מצב Git ובדיקות אחרונות

- תיקון rustfmt הקודם נמצא ב-`dd4d33e`; תיקון extractor נמצא ב-`92b09e410d98473502be531992a35f9d7100709d`, ו-format-only ב-`028319268d56eb2492ed36a16d861f02c2bcfcc7`. timeout fix נמצא ב-`4cb14433782d159d9f2fd24c6511af3a6370af8d`.
- [Rust CI 41360259748](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360259748) עבר על commit `3c8387f894c3c5bbd429ee98f88f42c9b78364d3`, event מסוג push, conclusion=`success`. זהו commit הקוד האחרון שנבדק; commits התיעוד המאוחרים יותר דורשים CI על HEAD העדכני. כל חמשת ה-jobs עברו: שלושת Quality jobs (Ubuntu/macOS/Windows), fixed-point reproducibility ו-negative unwrap quality gate. בכל שלוש הפלטפורמות עברו Formatting, Clippy, unit/integration tests, doc tests, release build ו-repeat clean build + metadata comparison.
- [Manifest scan 41355862823](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862823) עבר בעבר: 6 sources, 18,854 records, 35,356 resolved local edges, 413 unresolved local edges, 5,423 external references; שתי סריקות byte-identical. [Artifact](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862823/artifacts/13103227452). 413 unresolved edges עדיין לא סווגו סופית.
- [Manifest scan 41360202733](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733), על commit `dd4d33e172f816bdf2fdbb8e70ab53195821a2e2`, הסתיים ב-success. Artifact [13105540442](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733/artifacts/13105540442) מכיל manifest 70,097,869 bytes, SHA-256 `304cd6fe197726cf21a8fe2a7ba1e4b38d50e9aef2ae6da8a7fbe3ec5f8c8c47`, זהה byte-for-byte ל-13103227452. הסריקה דטרמיניסטית, אך 413 unresolved edges נשארו ודורשים triage ותיקון extractor.
- [Legacy runner 41356048158](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41356048158), על target commit `8ccd6611c19a87abf5e7934c9eb6b92eb992362a`, בוטל אחרי 60 דקות עקב `timeout-minutes: 60`; smoke עבר אך full-corpus לא הושלם. ה-timeout הוגדל ל-360 דקות ב-`4cb14433782d159d9f2fd24c6511af3a6370af8d`.
- [Legacy runner 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) על ה-workflow המתוקן: smoke עבר, full-corpus עדיין in progress. אין עדיין report מלא או artifacts.
- [Legacy full-corpus run קודם 41349079957](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41349079957) בוטל בעבר ואינו ראיית baseline replay מלאה.
- תיקון rustfmt לא שינה לוגיקת scanner, נתוני מקור, אלגוריתמי עימוד או baselines.
- [Rust CI 41361439397](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361439397) עבר על commit 028319268d56eb2492ed36a16d861f02c2bcfcc7: כל חמשת ה-jobs הצליחו. Ubuntu/macOS/Windows עברו Formatting, Clippy, unit/integration tests, doc tests, release build ו-repeat clean build; fixed-point ו-negative unwrap gate עברו.
- תיקון extractor ב-xtask/src/main.rs: JavaScript tokenizer מוגבל מדלג על comments ומבדיל בין string/template; מזהה static fetch/require/import/from/new URL, שדות asset מוגדרים ומערכי font/file/asset; בודק attributes ו-CSS url() בתוך literal/template; דוחה extension-only ו-directory-only fragments. נוספו unit tests חיוביים ושליליים. זה lexer ממוקד ולא parser מלא של JavaScript; dynamic runtime references עדיין דורשים inventory/owner review.
- [Legacy runner 41356048158](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41356048158) בוטל לאחר 60 דקות בדיוק עקב timeout-minutes: 60, לא עקב push. ה-timeout הוגדל ל-360 דקות ב-commit 4cb14433782d159d9f2fd24c6511af3a6370af8d.
- [Legacy runner 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) על commit ה-timeout המתוקן: checkout, dependencies, Chromium ו-smoke עברו; full-corpus comparison עדיין in progress. אין report מלא או artifact מלא עדיין.
- [Manifest scan 41361439336](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361439336) על commit 028319268d56eb2492ed36a16d861f02c2bcfcc7 עדיין in progress; אין עדיין ספירות/edge diff עדכניים.

- בוצע forensic audit ל-artifact הקודם: 413 unresolved edges ב-64 records, 270 ערכים ייחודיים. 118 הם extension-only, 62 inline markup fragments, 7 directory tokens (false-positive candidates); 9 environment executable paths ו-27 test commands/modules; 173 file-like/font edges עדיין דורשים אימות. הדו״ח [asset-edge-triage-41355862823.md](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/blob/main/docs/provenance/asset-edge-triage-41355862823.md) מצביע על חולשה ב-`extract_quoted_strings`: סריקת כל מחרוזות JS ללא lexer/הקשר תחבירי. אין להסיק שכל 413 edges שגויים או חסרים.
- הוחלט לא לשנות את manifest או לסמן edges כ-resolved באופן גורף. יש לתקן extractor עם positive/negative fixtures, לשמר את ה-manifest הקודם כ-baseline, ולהשוות edge diff בריצה חדשה.

## ראיות ADR

### Fixed-point
- 27,627 בדיקות Fraction מדויקות עברו ב-[CI 41349895727](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41349895727).
- זו הוכחת מודל ניסויי בלבד; עדיין אין בחירת scale סופית או מימוש Rust.

### Hebrew shaping / bidi
- HarfRust 0.15.0 + `unicode-bidi 0.3.18` עברו probe.
- השוואה ל-HarfBuzz 15.0.0: 5/5 דוגמאות עבריות טהורות התאימו בדיוק ב-[run 41348678930](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41348678930).
- חסרים mixed-direction run segmentation, source-span/cluster round-trip מלא, בדיקות glyph חסר וכל inventory הגופנים של הפרופילים.
- HarfRust הוא מועמד ניסויי בלבד; לא להוסיף ל-production workspace עדיין.

### Tauri
- [Run 41348380250](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41348380250) עבר build, Clippy ובדיקות generation/IPC gate ב-Windows.
- חסר invoke round-trip אמיתי דרך WebView, save/load, file dialogs ו-progress events.

### PDF Type0 / ToUnicode
- [Run 41350088448](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41350088448) עבר: Type0, ToUnicode, embedded/subset CID TrueType, 30 glyph mappings, repeat manifest זהה ו-MuPDF extraction מדויק לשלוש שורות.
- Poppler raw/layout אינם תואמים לסדר הלוגי; הם נשמרים כאבחון, לא עוברים.
- זה probe בן עמוד אחד בלבד. printpdf 0.12.8 הוא מועמד prototype, לא backend מאושר.

## סדר הפעולות הבא

1. לבדוק את סיום [Manifest scan 41361439336](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361439336), להוריד artifact, ולהשוות edge-by-edge ל-41360202733. לתעד מה הוסר, נשאר, נוסף או שינה resolution. לסווג כל unresolved אמיתי מול false positive; אין לסגור provenance על סמך scan pass בלבד.
2. להמשיך לעקוב אחרי [Legacy runner 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) עד סיום full-book corpus; לבדוק report, DOM, geometry, ledger ו-provenance artifacts מול baseline המוצמד. timeout 360 דקות; אין לשנות baseline/tolerance ואין לבטל את הריצה.
3. קוד extractor אומת ב-[Rust CI 41361439397](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361439397). לאחר עדכון מסמכי המעקב יש לבדוק את ה-CI החדש על HEAD הסופי.
4. להשלים provenance, source-of-truth matrix, scanner, manifest ו-parity matrix רק אחרי ראיות עדכניות ו-triage.
5. להשלים ניסויי fixed-point, shaping/bidi, Tauri ו-PDF ולתעד במפורש מה עבר ומה עדיין חסר.
6. משימות 1–2 בלבד סגורות; שלב א נשאר פתוח. אין להתחיל משימה 11 לפני שכל שערי שלב א נסגרו במפורש.

## קבצים שחובה לעדכן בכל הודעת עבודה

1. `docs/worklog.md` — יומן מפורט, commits, failures, fixes ו-run links.
2. `תוכנית_עבודה_לסוכן_AI_100_משימות.md` — טבלת יומן הביצוע החי בראש הקובץ וסימון checklist רק על סמך ראיות.
3. `docs/handoff/current-state.md` — תמונת מצב קצרה ומדויקת להמשך שיחה חדשה.

## כללי המשכיות

- מאגרי המקור הישנים לקריאה בלבד.
- לא לערבב V21/V23.
- אין להוסיף תלות ניסיונית ל-production לפני ADR מאושר.
- אין לסמן task כ-done בגלל קוד/מסמך בלבד; נדרשת בדיקת קבלה תואמת.
- אין להמציא תוצאות בדיקה. אם workflow pending, להשאירו pending ולתת קישור.
- סביבת המחשב המקומית בשיחה הקודמת לא כללה `cargo`/ `rustc`; אימות Rust נעשה ב-GitHub Actions.


## עדכון forensic — 2026-11-02

- [Manifest 41361439336](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361439336) הושלם וה-artifact הושווה ל-41360202733: 18,854 record IDs נשמרו ו-source hashes זהים; 86 records שינו edges/status, 486 edges הוסרו ו-2 נוספו. unresolved ירד מ-413 ל-157. הסריקה אינה מתקבלת כ-final.
- ה-diff חשף false negatives: 174 references של CSS font filenames עם רווחים (35 שמות ייחודיים) הוסרו, משום ש-normalizer דחה whitespace; וכן imports סטטיים ללא סיומת כגון ./lib/harness לא נפתרו לקובץ .js יחיד.
- תיקון נוסף: [commit 56ab319](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/56ab3190cb2817864533eb5c536f55640f2a5f4d) הוסיף CSS-only whitespace acceptance ו-resolution שמרני של extensionless JS imports. ה-CI של commit זה נכשל עקב syntax error ב-guard; הוא תוקן ב-[commit 4d4a5c5](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/4d4a5c557289f66353aa5eb482e0b2754cf7553c).
- [Rust CI 41363057774](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363057774) ו-[Manifest scan 41363057769](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363057769) על 4d4a5c5 עדיין רצים. יש להתייחס לסריקה זו כקובעת רק לאחר השוואת artifact.
- [Legacy runner 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) עדיין full-corpus in progress; smoke עבר, אין report מלא.
- שלב א פתוח; משימות 1–2 בלבד סגורות. אין לשנות baseline/tolerances ואין להתחיל שלב ב.


## תיקון מבני אחרון — 2026-11-02

- CI 41363057774 זיהה קטע normalizer משוכפל אחרי מודול tests, ולכן commit 4d4a5c5 לא היה תקין. הקטע הוסר ב-[commit 99e4266](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/99e42666f5a6b1fb326b33ac0cab2062e1cea986).
- אימות מבני של קובץ xtask/src/main.rs לאחר התיקון: 1,848 שורות, הגדרה אחת של resolve_local_asset_path ומודול tests יחיד. זהו אימות טקסטואלי בלבד; קומפילציה טרם אושרה.
- [Rust CI 41363244229](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363244229) ו-[Manifest 41363244210](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363244210) על commit 99e4266 queued/pending. יש להמתין לתוצאות אלה לפני שימוש ב-manifest החדש.
- [Legacy runner 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) עדיין full-corpus in progress; אין report מלא.
- שלב א פתוח; משימות 1–2 בלבד סגורות. אין לשנות baselines/tolerances ואין להתחיל שלב ב.


## עדכון formatting — 2026-11-02

- [CI 41363244229](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363244229) נכשל רק ב-rustfmt על assert_eq אחד; התיקון formatting-only ב-[commit 17c68ea](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/17c68ea72860798d5f1f31b2c02f9acca7a06a22).
- [Rust CI 41363321746](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363321746) ו-[Manifest 41363321758](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363321758) queued/pending על 17c68ea; אין עדיין תוצאת קבלה.
- Legacy runner 41361263075 נשאר full-corpus in progress; אין report מלא. שלב א נשאר פתוח.


## Manifest forensic classification — 2026-11-02

- Completed scan [41363321758](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363321758), artifact [13106386301](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363321758/artifacts/13106386301), compared to [41360202733](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733), artifact [13105540442](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733/artifacts/13105540442).
- 18,854 IDs stable; no source hash changes. Edges 41,192→40,881; resolved 35,356→35,359; unresolved 413→139. Delta: 313 removed edges, 2 added; resolved +18/-15; unresolved -274.
- All 139 unresolved edges across 17 records were inspected: 133 point to historical archive paths absent from the pinned IMUD tree; 3 point to Gold engine scripts referenced by renderer.html but absent from the pinned Gold tree; 2 are npm package bin entries misread as local paths from package-lock; 1 is a bare npm module specifier misread as a local path. Thus 136 are genuinely unresolved against pinned trees, 3 are extractor false positives.
- Do not mark historical archive references resolved merely because they are historical. Decide whether those archive directories must be restored to the pinned source or explicitly remain historical-unresolved. Gold's three missing scripts require owner decision/restoration or explicit documented absence.
- CI 41363362478 passed all 5 jobs. Follow-up extractor fix [1a1ce26](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/1a1ce26b6fefa3db40decc304b86353284e466d6) excludes bare Node package specifiers from local module resolution and excludes package-lock npm bin entries from repo-local data extraction; two tests added. CI [41367799248](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41367799248) and manifest [41367799385](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41367799385) pending.
- Legacy full corpus [41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) remains in progress after smoke passed; no full report yet.
- Stage A remains open; do not start Stage B.


## npm false-positive patch formatting

- Rust CI [41367799248](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41367799248) failed only rustfmt on two formatting locations; other two CI jobs passed.
- Formatting-only fix committed as [4af38d4](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/4af38d4779c8624924af1063e916246bcd71dab3).
- Validation runs [41367871354](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41367871354) and [41367871327](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41367871327) queued.
- Legacy full-corpus run [41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) remains in progress.


## המשך מצב — 2026-11-02

- ראש הקוד הנוכחי: [218d50a](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/218d50abc1758fa9b187f3d39a9882d876ee7321). תיקוני extractor אחרונים: font inventory basenames נשמרים באמצעות asset normalizer; package-lock.json סורק רק property בשם resolved כדי לשמור URLs חיצוניים בלי להחזיר npm bin false positives.
- Manifest [41367871327](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41367871327), artifact [13108185810](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41367871327/artifacts/13108185810), לעומת baseline [41360202733](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733): 18,854 compatIds יציבים, 0 source hash changes; assetDependencies 41,192→40,626; resolved 35,356→35,221; unresolved 413→35. 35 unresolved: 32 paths היסטוריים בארכיונים ו-3 scripts חסרים ב-Gold. תוצאות אלה הן של 4af38d, לפני תיקון inventory/lockfile, ולכן נדרשת סריקה חוזרת.
- CI [41369416363](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369416363) גילה 2 unit test failures (font inventory, package-lock resolved URL); התיקון ב-7bcd686. CI [41369485795](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369485795) גילה rustfmt בלבד; תוקן ב-218d50a.
- ריצות עדכניות על 218d50a: [Rust CI 41369525393](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369525393) queued; [Manifest 41369525487](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369525487) pending.
- [Legacy full corpus 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) עדיין in progress בשלב full-book corpus comparison; smoke עבר, אין report/artifact מלא. להשאיר פעיל.
- Stage A פתוח; tasks 1–2 בלבד סגורות. אין Stage B, אין baseline/tolerance changes, אין סימון parity/provenance/manifest כהושלם עד לקבלת הראיות.


## תיקון fetch asset — 2026-11-02

- ראש קוד extractor: [73557d4](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/73557d41d240698c847e5e4d88fafcf82235e4d3). fetch URL מטופל כ-asset reference; require/import מטופלים כ-module specifiers. זאת בעקבות unit test שהראה שהנתיב fonts/fonts.json הושמט.
- ב-CI [41369525393](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369525393) עברו rustfmt ו-Clippy בכל שלוש הפלטפורמות; 20/21 unit tests עברו. שתי בדיקות שתוקנו קודם (font inventory ו-package-lock external URL) עברו; נותר כשל fetch יחיד שתוקן ב-73557d4.
- אימות עדכני: [Rust CI 41369621550](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369621550) ו-[Manifest 41369621509](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369621509) pending. Manifest קודם [41369525487](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369525487) על 218d50a עדיין in_progress אך אינו כולל את תיקון fetch.
- [Legacy full corpus 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) נשאר in_progress; smoke עבר, אין report מלא.
- Stage A פתוח; משימות 1–2 בלבד סגורות. אין Stage B, אין שינוי baseline/tolerance.


## קבלת CI — 2026-11-02

- [Rust CI 41369673903](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369673903) עבר בכל חמשת ה-jobs על קוד extractor 73557d4: Linux, macOS ו-Windows עברו Formatting, Clippy, 21 unit tests, doc tests, release build ו-repeat clean build/metadata; fixed-point reproducibility ו-negative unwrap gate עברו.
- [Manifest 41369621509](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369621509) עדיין in_progress ביצירת manifest canonical פעמיים; artifact טרם זמין.
- [Legacy full corpus 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) עדיין in_progress בשלב full-book corpus comparison אחרי smoke מוצלח; timeout 360 דקות. אין report מלא.
- Stage A פתוח; משימות 1–2 בלבד סגורות. נדרשים manifest edge diff, הכרעה על archive/Gold unresolved ו-full corpus report. אין Stage B ואין שינוי baselines/tolerances.


## מצב עדכני — 2026-11-02

- קוד ראשי: [3b4268d](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/3b4268d0e3e806729248aef7bbe0b2d95d1293a2).
- [Rust CI 41374863123](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41374863123) עבר בכל חמשת ה-jobs על 3b4268d.
- [Manifest 41374863178](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41374863178) על 3b4268d עדיין in_progress. artifact קודם 73557d4: 18,854 IDs יציבים, 0 source hash changes; asset edges 41,192→40,877; resolved 35,356→35,359; unresolved 413→136. ה-unresolved: 132 archive edges (121 unique paths בשבעה תתי-עצים), 3 Gold scripts חסרים, והפניה אחת fetch ב-test שנפתרה ביחס שגוי. אין להסיק שכל תיקיות archive חסרות.
- השוואת artifact קודם מצאה גם 15 resolved edges שנעלמו ו-59 unresolved removals שעדיין דרשו review; לכן 73557d4 לא התקבל. תיקוני 3b4268d מוסיפים זיהוי של nested templates, path.join, page.goto, readFileSync fallback, DOM href/src/poster, ו-fetch root semantics ל-browser tests. יש לאשר רק לאחר diff של artifact חדש.
- [Legacy full corpus 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) עדיין in_progress; smoke עבר, full-book comparison טרם הסתיים ואין report/artifacts.
- Stage A פתוח; tasks 1–2 בלבד סגורות. אין Stage B ואין שינוי baseline/tolerance.


## Handoff update — manifest triage 2026-11-02

- Code at scan: [526d6a0](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/526d6a04508be4145b09efaf25424ca59fb7ef51); Rust CI [41376496424](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41376496424) passed.
- Manifest [41376496462](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41376496462) passed; artifact [13111866073](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41376496462/artifacts/13111866073) compared to baseline 13105540442: records/compatIds 18,854 identical; source hashes unchanged; edges 41,192→40,940; resolved 35,356→35,387; unresolved 413→161. No previously resolved edge was lost.
- 161 unresolved: IMUD 147 edges / 133 unique assets missing from seven partial archive subtrees; Gold 13 edges / 4 paths (three missing engine scripts + ten references to a missing root-level text fallback); Orange one path escaping repository root. Detailed triage: [asset-edge-triage-41376496462.md](../provenance/asset-edge-triage-41376496462.md).
- These are not currently classified as extractor false positives. Source-owner decisions/restoration required before manifest/provenance acceptance. No suppressions, baseline changes, or tolerance changes.
- Legacy full corpus [41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) still in progress at full-book comparison; prior smoke/renderer/DOM/ledger passed; report pending.
- Stage A: tasks 1–2 closed, 3–10 open. Stage B not started.


## Stage A provenance gate update — 2026-11-02

- Six pinned commits and six tree SHAs were revalidated against GitHub: all match.
- All **47/47** pinned file Git blob SHA-1 values match the corresponding pinned trees (Orange 15, Green 7, Gold 7, Yellow 7, IMUD 6, Typesetter_doc 5).
- Orange lineage reference commits for Green import, Gold reference and common ancestor are retrievable; see `docs/provenance/source-of-truth-matrix.md`.
- The five newly uploaded source files were re-hashed and match the existing unattributed attachment records exactly. Their origin remains unresolved; they remain a separate source set.
- Stage A tasks 1–3 are closed. Tasks 4–10 remain open. Stage B remains not started.


## Task 4 progress — 2026-11-02

- Five pinned executable-profile default settings files compared: union 267 keys, 246 common to all, 21 presence differences, zero differing values for keys shared by profiles.
- Grouping: two Green-only engine keys, one Orange-only header key, eleven Gold+IMUD AQ keys, and seven Orange+Gold+IMUD decoration/divider keys.
- Absence is not false/zero/empty; preserve profile-specific presence semantics. Full matrix in `docs/provenance/source-of-truth-matrix.md` §10.
- Task 4 remains open; next inventory must cover presets, assets, tests and other profile-owned data.


## Task 4 test inventory progress — 2026-11-02

- Test harnesses are profile-specific. Orange has seven npm test suites; Green has no npm test script; Gold owns V21 scripts/workflows for baseline/quick/determinism; Yellow has tests but no root package.json; IMUD has a separate quick-layout and feature suite.
- Orange/Green share eight fixture paths; seven byte-identical, one differs in six header-type settings. Full evidence: `docs/provenance/source-of-truth-matrix.md` §12.
- Task 4 remains open. Structural path counts do not establish coverage; next acceptance work is to map compatibility requirements to profile-owned tests.
- Legacy full corpus run 41361263075 status: `in_progress`; last job remained in full-book corpus comparison.


## Workflow checkpoint — 2026-11-02 03:02 UTC

- Main/docs commit: `1a0cb91d0df73c22a52be8e0f23f37cf27a3f955`.
- Code commit `526d6a0` has passing dedicated Rust CI [41376496424](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41376496424).
- Docs-triggered Rust CI [41380773019](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41380773019) and manifest scan [41380737375](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41380737375) are in progress. The latter is a second scan of unchanged source code/pins and should be compared to artifact 13111866073 for reproducibility once it finishes.
- Full corpus runner [41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) remains in progress; no full-corpus result yet.


## Handoff update — preset inventory and extractor follow-up — 2026-11-02

- Task 4 advanced: all five pinned profiles share the same three root `setting_sets/` blobs (249 keys each); the three names still differ in 16 values each against `פרשנות`. IMUD also contains a nested V20.5 snapshot family with 249-key files that differ from current root defaults (24 missing keys, 9 additional keys, 66–82 changed values). Its pinned settings manager does not load that nested folder. V19/V20.4 archive defaults were also compared. Evidence: `docs/provenance/source-of-truth-matrix.md` §13.
- Latest completed manifest at `53412ee`: run [41382627753](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41382627753), artifact [13113743541](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41382627753/artifacts/13113743541). Baseline identity/hash stable (18,854 records, 0 sourceSha256 changes); edges 41,192→40,954; resolved 35,356→35,398; unresolved 413→164. Three new unresolved edges are archived IMUD `doc_full.txt`/`doc_part1.txt` references; these files are absent from the pinned tree. The 3 previously resolved edges lost require review: Gold `SCRIPTS_DIR = path.dirname(__filename)`, IMUD runtime `root` parameter in `hasFonts(root)`, and Yellow runtime `root` parameter in `runTree(browser, root, plan)`.
- Extractor correction for the Gold static alias was committed as [24d58bb](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/24d58bb9fbfe03af3c3dbd2b03351abd0d601f72): recognize `path.dirname(__filename)`/static script-directory bindings while continuing to reject runtime root parameters. Rust CI [41384362259](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41384362259) passed all five jobs. Manifest rerun [41384362288](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41384362288) is still in progress; do not accept the new extractor result until its artifact confirms the Gold dependency is restored and runtime-root false positives remain absent.
- Legacy full-corpus run [41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) remains in progress at the full-book corpus comparison step; no final report/artifact yet.
- Stage A tasks 1–3 closed; task 4 remains open with partial settings/preset/test ownership mapping; tasks 5–10 open. Stage B not started.


## מצב עדכני — 2026-11-02, אחרי e2e2831

- Main code commit: [e2e2831](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/e2e283151c9ae5bc25517a3fdfe6c867219f491e).
- Rust CI [41385039659](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41385039659): success בכל חמשת ה-jobs.
- Manifest [41385039519](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41385039519), artifact [13115046214](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41385039519/artifacts/13115046214): success. 18,854 records, 40,954 asset edges, 35,399 resolved, 163 unresolved edges / 115 unique paths, zero sourceSha256 changes.
- Extractor now records the actual `fonts/<file>` URL for static `PRIORITY_FONT_FILES` arrays and excludes generated `writeFileSync` targets such as `fuzz_last.json`. Static `path.dirname(__filename)` aliases were separately verified in manifest run [41384549551](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41384549551).
- Historical archive recovery: seven exact read-only refs are pinned in `docs/provenance/historical-archive-source-pins.json`. Their trees contain 98 candidate path targets; 17 unique targets were not found in the corresponding exact commits. Do not mark the 98 resolved in the primary manifest until they are represented as separate source profiles or exact mappings. V20.2 and V20.3 have archive/branch core-file differences; V20.3 work_v4 data files remain missing.
- Full-corpus runner [41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) remains in progress at the full-book corpus comparison step. No final report/artifact yet; timeout remains 360 minutes. Do not cancel or infer a result.
- Stage A tasks 1–3 closed; task 4 remains open with substantial but partial provenance mapping; tasks 5–10 open. No Stage B. No baseline/tolerance change. V21 and V23 remain separate.

Next: decide whether to add V19/V20.1–V20.5/V21 historical trees as separate manifest profiles; obtain or explicitly waive the 17 unresolved targets; then continue the remaining Stage A gates.
