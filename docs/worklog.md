# יומן עבודה — tora-typesetter Rust

## מצב נוכחי

- מאגר: `SUGINACH/Tora_Book_Typesetter_Rust`
- ענף עבודה: `main`
- נקודת מוצא: `2a6c4f11075a75a883c284c484e26db6db9e7828`
- commit אתחול workspace: `0b76c2e8cdda388f6e6ef7b843db21656a880d90`
- commit הקמת CI: `ee5dc4c1325fcd32f0e782262b63600487ca755b`
- שלב פעיל: שלב א — משימות 1–10; משימות 1–2 הושלמו, 3–9 באימות, 10 בניסויים.

## החלטות והבהרות

1. תוכנית 100 המשימות היא מסגרת הביצוע המחייבת, כולל תלויות וראיות סיום.
2. תוכנית הפיתוח המקיפה כוללת בנספח א' חלוקת חבילות מוקדמת יותר. תוכנית 100 המשימות מציגה גבולות אחריות מפורטים יותר. בשלב האתחול נוצרו רק חבילות הבסיס `model`, `geometry`, `engine-core` ו-`xtask`; הרחבתן תיעשה בשלבים התלויים ולא ניצור מראש חבילות ריקות רבות.
3. toolchain הוצמד ל-Rust 1.99.1, הגרסה היציבה האחרונה שאומתה בעת יצירת השלד (2026-11-01).
4. אין עדיין לוגיקת עימוד, מדידת טקסט, shaping, pagination או renderer. אין להציג את השלד כמנוע עובד.
5. אין לבצע שינויים במאגרי המקור הישנים; הם מקורות קריאה בלבד. פרופילי V21 ו-V23 יישארו מבודדים.

## רישום ביצוע

### משימה 1 — אתחול workspace ונעילת toolchain

- [x] Cargo workspace עם resolver 2 וחבילות בסיס.
- [x] `rust-toolchain.toml` מוצמד ל-Rust 1.99.1 עם rustfmt ו-Clippy.
- [x] `Cargo.lock` כולל את כל חבילות ה-workspace המקומיות.
- [x] פרופיל release עם overflow checks ושלד `xtask`.
- [x] `cargo build --release --locked` עבר ב-Windows, Linux ו-macOS.
- [x] build נקי חוזר והשוואת workspace metadata עברו; אין שינוי metadata בין הריצות.

**סטטוס משימה 1: הושלמה.** ראיה: [CI 41347908551](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41347908551), commit `18b73bd267ef8b4c8e5afe416dd86ffcd20f4f40`.

### משימה 2 — CI ומדיניות איכות

- [x] workflow רב־פלטפורמי ל-Windows, Linux ו-macOS.
- [x] rustfmt, Clippy עם warnings כשגיאות, unit/integration tests ו-doc tests.
- [x] נעילת תלויות באמצעות `--locked`.
- [x] איסור `unsafe`, `unwrap()` ו-`expect()` באמצעות workspace lints.
- [x] probe שלילי הוכיח ש-Clippy חוסם `unwrap()`.
- [x] כל שלושת jobs של מערכת ההפעלה הסתיימו בהצלחה; גם בדיקות איכות נפרדות עברו.

**סטטוס משימה 2: הושלמה.** ראיה: [CI 41347908551](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41347908551).

## תיקוני ביקורת לפני CI

- בביקורת ה-diff של commit הקמת ה-workspace התברר ש-`Cargo.lock` הראשוני לא כלל רשומות לחבילות המקומיות. נוספו הרשומות המפורשות לפני קבלת תוצאת CI, כדי ש-`--locked` לא ייכשל בעדכון lockfile אוטומטי.
- אין עדיין טענה שה-build או ה-probe עברו; ההכרעה תיעשה לפי GitHub Actions בלבד.


## עדכון ביצוע — 2026-11-01

### תוצאת CI בסיסית

ה-commit `18b73bd267ef8b4c8e5afe416dd86ffcd20f4f40` עבר את workflow `Rust CI` במלואו:
- Windows, Linux ו-macOS: toolchain, rustfmt, Clippy, unit/integration tests, doc tests, release build ו-build חוזר — כולם success.
- ה-probe השלילי נכשל בכוונה בתוך ה-job שלו, וה-job עצמו עבר לאחר שאימת את אבחון ה-lint הצפוי.
- ניסוי fixed-point המבוסס על שברים מדויקים עבר שחזור.
- לא הוסקה מכך בחירה סופית של יחידת fixed-point.

### ניסויים שנותרו במשימה 10

- HarfRust 0.15.0 + unicode-bidi 0.3.18: ריצת probe ראשונית עברה; אין בכך עדיין הוכחת שילוב bidi runs או בחירת shaper.
- Tauri 2: build ובדיקות IPC עברו בשלבים הראשונים; release build של Windows עדיין בבדיקה בריצה 41347404787.
- PDF Type0/ToUnicode: prototype מתקדם עם printpdf ו-ActualText; ריצה 41347908683 עדיין בבדיקה. אין לאשר backend לפני חילוץ עצמאי של כל הקורפוס והשוואה מדויקת.
- ADRs נשארים proposed עד לסגירת המדידות והחלופות.



## עדכון ביצוע — 2026-11-01, 21:02

### משימות 1–2: סגורות

ה-commit `18b73bd267ef8b4c8e5afe416dd86ffcd20f4f40` עבר את [Rust CI 41347908551](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41347908551) בכל שלוש מערכות ההפעלה. כל gates של formatting, lint, tests, doc tests, release build, build חוזר ו-metadata consistency עברו. ה-probe השלילי נכשל בכוונה על `unwrap` וה-job אימת זאת בהצלחה.

### Provenance, ownership ו-manifest

- שישה מאגרי מקור מוצמדים ל-commit מלא; חמשת הקבצים שסופקו בשיחה (`style.css`, `decorations.js`, `typesetter.js`, `index.html`, `app.js`) נבדקו מחדש מהקבצים הממופים. אורכם ו-SHA-256 שלהם תואמים בדיוק לרשומות `unattributedUserAttachments`; אין להסיק מהם branch/repository מקור.
- מטריצת מקור האמת מפרידה בין Orange V23, Green V23, Gold V21, Yellow V10 ו-IMUD; נוסף test guard שמונע ערבוב זהויות fixture בין V21 ל-V23.
- `xtask` מפיק manifest קנוני פעמיים. נוספו בדיקות התאמה מדויקת ל-runner evidence, שדות schema, hash, בעלות, test IDs, רישוי fonts ומסמכי spec. ריצה אחת נכשלה משום שקובץ TSV הכיל את התווים המילוליים `\\t` במקום tab אמיתי; הקובץ תוקן, ונוספה הפעלה אוטומטית מחדש כשקובץ ה-evidence משתנה.
- [סריקת manifest הנוכחית](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41349723140) עדיין רצה.

### Legacy runner והקורפוס

- ה-smoke הקודם עבר: [41344525335](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41344525335).
- ה-runner המלא נועל במפורש את `very_long_test_text.txt`, `default_settings.json`, CSS הגופנים וה-baseline; ה-timeout הוגדל ל-900,000ms וה-report מאמת שה-settings הפעילים הם הקובץ המוצמד.
- [full-corpus run 41349079957](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41349079957) עדיין רץ. אין עדיין מסקנה לגבי התאמה ל-baseline.

### ניסויי ADR

- **Fixed-point:** נוסף מודל `Fraction` מדויק עם 27,627 בדיקות דטרמיניסטיות: px↔pt, percentages, font units, signed addition, overflow boundaries ו-tie policy. תוצאות נשמרות ב-`experiments/fixed-point-scale-results.json`. אלה עדיין בדיקות מודל Python, לא מימוש Rust; בחירת scale נשארת פתוחה. [CI אחרון](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41349895727) רץ.
- **Shaping/Bidi:** HarfRust 0.15.0 מול HarfBuzz 15.0.0 השיג התאמה מדויקת ב-5/5 דוגמאות shaping עבריות טהורות. זה עדיין לא מוכיח segmentation של mixed-direction runs או source-span mapping מלא. [Run 41348678930](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41348678930).
- **Tauri:** Windows formatting, debug/release builds, Clippy ובדיקות generation gate עברו; נוספו ולידציות ל-profile ID, job ID, generation ו-SHA-256. עדיין לא נבדק invoke round-trip אמיתי דרך WebView, וגם לא save/load, file dialogs ו-progress events. [Run 41348380250](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41348380250).
- **PDF:** Poppler ו-MuPDF חשפו חילוץ עברי לא תקין בניסוי הראשון. ה-PDF probe שונה כך ש-`BDC/EMC` מקיפים text objects, וה-glyphs מותאמים חזרה לתווי המקור דרך cmap כדי להחזיר סדר Unicode לוגי בלי לשנות מיקום glyph. נוספו בדיקות לסדר marks ול-ligature fallback. [Run 41349766772](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41349766772) עדיין רץ; אין אישור backend.

### שערי קבלה שנותרו לשלב א

1. לקבל full-corpus report ולהשוות metrics ל-baseline בלי לנרמל הבדלים.
2. לקבל manifest כפול זהה, JSON תקין, ושש רשומות המקור תואמות ל-pins.
3. להשלים fixed-point CI ולהשאיר פתוחות בדיקות Rust-level שעדיין אינן חלק מהשלב.
4. לקבל חילוץ PDF לוגי מדויק ב-MuPDF/Poppler; אם לא עובר, לתקן או לפסול את ה-backend.
5. לוודא שה-Tauri ו-shaping ADRs משקפים את מגבלות הניסוי, ולא להכריז על בחירת production לפני השלמתן.

אין מעבר לשלב ב עד שכל שערי שלב א נסגרים בראיות.


## עדכון המשכיות — 2026-11-02

### מצב Git ו-CI

- ענף: `main`; commit אחרון לפני עדכון מסמכים זה: `6a2afeaf49a764bcdb79af63739a3dca524833bb`.
- תיקון קריטי: `xtask/src/main.rs::normalize_asset_reference` הכיל guard לסינון מחרוזות דינמיות ללא `return None` וסוגר בלוק. הדבר גרם ל-`unclosed delimiter` ולכשל הן ב-Rust CI והן ב-manifest scan. תוקן ב-commit `e8aac03f4486a112bebf3a21fca954264e29c8e4`.
- לאחר תיקון התחביר, CI חשף שתי סטיות rustfmt ב-`xtask/src/main.rs`; הן תוקנו ב-commit `6a2afeaf49a764bcdb79af63739a3dca524833bb`.
- [Rust CI 41355862773](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862773) נפתח אחרי תיקוני syntax ו-rustfmt; היה בתהליך בעת עדכון זה.
- [Manifest scan 41355862823](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862823) נפתח על אותו commit; היה pending/in progress בעת עדכון זה.
- הכשל הקודם [Rust CI 41350464541](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41350464541) נבע מתחביר לא סגור; [manifest 41350465293](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41350465293) נכשל באותו compile. אין להסתיר ריצות כושלות אלה; הן תועדו ותוקנו.
- שונתה מדיניות concurrency של ה-Legacy runner: כל ריצה מזוהה לפי commit, ו-`cancel-in-progress: false`. מטרת השינוי היא לאפשר ל-full-corpus replay להסתיים גם כאשר נדחפים commits נוספים. ה-workflow ירוץ מחדש על commit זה.

### ראיות טכנולוגיות מעודכנות

- **Fixed-point:** ניסוי Fraction מדויק עם 27,627 בדיקות דטרמיניסטיות עבר ב-[CI 41349895727](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41349895727). זהו מודל ניסויי, לא מימוש Rust ולא בחירת scale סופית.
- **Shaping/Bidi:** HarfRust 0.15.0 + `unicode-bidi 0.3.18` עברו; השוואה מול HarfBuzz 15.0.0 התאימה בדיוק ב-5/5 דוגמאות עבריות טהורות ב-[run 41348678930](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41348678930). Mixed-direction run segmentation ו-source-span round-trip עדיין חסרים.
- **Tauri:** [run 41348380250](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41348380250) עבר build/Clippy/tests ב-Windows. אין עדיין WebView invoke round-trip מלא, שמירה/טעינה, file dialogs ו-progress events.
- **PDF:** [run 41350088448](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41350088448) עבר: printpdf יצר PDF עם Type0, ToUnicode ו-font subset; 30 mappings; שתי יצירות PDF נתנו manifest זהה; MuPDF החזיר בדיוק את שלוש שורות הקורפוס. Poppler raw/layout נשמרו בדוח, אך אינם תואמים לסדר הלוגי בשל bidi post-processing. זה probe ממוקד בן עמוד אחד, לא ספר מלא ולא בחירת backend.
- **Legacy full corpus:** [run 41349079957](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41349079957) הסתיים ב-`cancelled`; אין תוצאת baseline replay מלאה. יש להריץ מחדש אחרי שינוי concurrency ולשמור את הדוח וה-manifest.

### משימת ההמשך המדויקת

1. לבדוק תחילה את [Rust CI 41355862773](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862773) ואת [manifest scan 41355862823](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862823).
2. אם אחד נכשל: להוריד job logs, לבודד את השגיאה, לתקן, לעדכן worklog/checklist/handoff ולדחוף rerun. אין להמשיך לתכונה חדשה לפני שה-gates הרלוונטיים ירוקים.
3. לוודא שה-legacy workflow החדש אכן מופעל על commit זה; אם לא, להפעילו ידנית. אין לבטל את full-corpus run.
4. להשוות את תוצאות Orange full corpus ל-baseline המוצמד בלי לנרמל הבדלים. אם יש הבדל, לתעד את כל המדדים והסיבות האפשריות; אין לשנות baseline כדי להשיג pass.
5. להמשיך לסגור משימות 3–10 לפי הסדר. בכל הודעת המשך לבצע יחידת עבודה שלמה אחת, לעדכן worklog, checklist ו-handoff.
6. אין להתחיל שלב ב לפני סגירה מפורשת ומגובה ראיות של שלב א.

### כללי עבודה שאסור לשנות

- `V21` ו-`V23` הם פרופילים/ענפים נפרדים; אין למזג ביניהם קוד, fixtures או baselines.
- מאגרי המקור הישנים הם לקריאה בלבד.
- אין להוסיף HarfRust, printpdf או ספריות ניסוי ל-production workspace לפני ADR והוכחות מלאות.
- אין להכריז על success על סמך קיום workflow או קובץ; רק conclusion=success ו-artifacts/metrics תואמים נחשבים ראיה.
- סביבת העבודה המקומית בשיחה הקודמת לא כללה `cargo`/ `rustc`; לכן אימות Rust נעשה ב-GitHub Actions, לא באמצעות טענת build מקומית.


## עדכון ביצוע — 2026-11-02, אימות ריצות ההמשך ותיקון rustfmt

### תוצאות מאומתות

- [Rust CI 41355862773](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862773) הסתיים בכשל בשלושת jobs של Quality (macOS, Windows, Ubuntu), כולם בשלב Formatting. יומני macOS מצביעים על סטיית rustfmt אחת ב-xtask/src/main.rs באזור resolved_asset_dependencies: rustfmt דורש את פתיחת בלוק הלולאה בשורה נפרדת. Clippy, tests, doc tests ו-release build דולגו בעקבות עצירת ה-job בשלב Formatting.
- באותה ריצה, Fixed-point scale experiment reproducibility עבר; גם Verify quality gate rejects unwrap עבר.
- [Manifest scan 41355862823](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862823) עבר במלואו: 6 מקורות מוצמדים, 18,854 רשומות, 35,356 resolved local edges, 413 unresolved local edges ו-5,423 external references. שתי סריקות יצרו קובץ זהה byte-for-byte. Artifact: [legacy-compatibility-manifest](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862823/artifacts/13103227452).
- ה-legacy full-corpus run [41349079957](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41349079957) הסתיים ב-cancelled בשלב השוואת Orange V23 full-book corpus ל-baseline. שלב ה-smoke והעלאת artifacts הושלמו, אך אין תוצאת full-corpus תקפה.

### תיקון

- תוקנה סטיית rustfmt היחידה שנצפתה ב-xtask/src/main.rs; לא שונו לוגיקת הסריקה, סיווג dependencies, נתוני מקור או baselines.
- תיקון זה הוא תיקון formatting בלבד. אין לסמן משימות 3–9 כסגורות על בסיסו; נדרשים CI ירוק ועדכון הראיות של full-corpus.

### פעולות המשך

1. להריץ Rust CI מחדש על ה-commit שמכיל את תיקון formatting ואת עדכון מסמכי המעקב; לוודא שכל שלושת ה-platform jobs עוברים את כל השלבים, ולא רק Formatting.
2. להפעיל מחדש את legacy full-corpus workflow על אותו commit, בלי cancel-in-progress ובלי שינוי baseline/tolerance; לשמור report, DOM, geometry, ledger ו-provenance artifacts.
3. לבדוק את כל 413 unresolved local edges לפי רשומות/פרופילים, ולסווג false positives לעומת תלות חסרה אמיתית. אין להמיר אותם ל-resolved ללא הוכחה.
4. משימות 1–2 נשארות סגורות; שלב א נשאר פתוח. אין להתחיל שלב ב.


## עדכון ביצוע — 2026-11-02, CI ירוק ובדיקות ארוכות פעילות

### אימות Rust CI לאחר תיקון rustfmt

- [Rust CI 41360259748](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360259748) הופעל על `main` ועל commit `3c8387f894c3c5bbd429ee98f88f42c9b78364d3`; הסתיים `success`.
- כל ארבעת ה-jobs עברו: Quality / Ubuntu, Quality / macOS, Quality / Windows, Fixed-point scale experiment reproducibility ו-Verify quality gate rejects unwrap (סה״כ חמישה jobs; שלושת ה-platform jobs עברו את כל השלבים).
- בכל שלוש הפלטפורמות עברו: pinned toolchain, Formatting, Clippy, unit/integration tests, documentation tests, release build ו-repeat clean build + workspace metadata comparison.
- לכן תיקון rustfmt ב-`xtask/src/main.rs` מאומת. אין שינוי במימוש האלגוריתמי או ב-baselines.

### ריצות ראיות שעדיין פעילות

- [Legacy runner 41356048158](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41356048158), על commit יעד `8ccd6611c19a87abf5e7934c9eb6b92eb992362a`: checkout, התקנת dependencies ו-Chromium, ו-smoke renderer עברו; שלב Orange V23 full-book corpus מול baseline עדיין `in_progress`. ה-workflow משתמש בקבוצת concurrency ייחודית ל-commit וב-`cancel-in-progress: false`; אין לבטל אותו. עדיין אין report סופי או artifacts זמינים.
- [Manifest scan 41360202733](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733), על commit `dd4d33e172f816bdf2fdbb8e70ab53195821a2e2`: כל ששת ה-checkouts ואימות commits עברו; יצירת manifest כפולה עדיין `in_progress`. אין להתייחס לסטטיסטיקות או ל-artifact כמאומתים עד לסיום.
- [Manifest scan 41355862823](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862823) נותרה ראיה קודמת תקפה ל-6 sources, 18,854 records, 35,356 resolved local edges, 413 unresolved local edges ו-5,423 external references, עם שתי סריקות byte-identical. 413 unresolved edges עדיין דורשים triage; אין לסווגם אוטומטית כ-false positives.

### מסקנת שלב

- משימות 1–2 סגורות; שער CI הרב-פלטפורמי עבר שוב על ה-HEAD המתועד.
- משימות 3–9 אינן נסגרות עדיין: manifest עדכני ממתין, 413 unresolved edges דורשים triage, ו-full-corpus replay עדיין רץ.
- משימה 10 נשארת בתהליך בגלל פערי shaping/bidi, Tauri WebView round-trip והיעדר PDF backend מאושר.
- שלב א נשאר פתוח. אין להתחיל שלב ב ואין לטעון שמנוע עימוד production קיים.

### הפעולה הבאה

1. להמתין לסיום 41356048158 ו-41360202733 ולבדוק את כל ה-artifacts וה-reports, בלי לעצור את הריצות.
2. לנתח את 413 unresolved local edges לפי source, profile, reference type ו-resolution reason; לתעד כל קטגוריה ורשומה בעייתית.
3. לעדכן את מטריצת parity וה-checklist רק לאחר שהראיות תומכות בסגירה מפורשת.


## עדכון ביצוע — 2026-11-02, forensic triage ל-413 unresolved edges

### בדיקת artifact והקוד

- הורד ונבדק ה-artifact של [Manifest scan 41355862823](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862823/artifacts/13103227452): 18,854 records, 413 unresolved local edges, 270 ערכי edge ייחודיים ב-64 רשומות מקור.
- חלוקה לפי מקור: IMUD 271, Orange V23 61, Yellow V10 48, Green V23 23, Gold V21 10. לא מערבבים את פרופילי V21 ו-V23.
- נמצאו false-positive candidates ברורים: 118 ערכי extension-only כגון `.json`/ `.svg`, 62 קטעי inline markup/SVG/HTML, 7 directory tokens. בנוסף: 9 נתיבי executable סביבתיים, 27 פקודות/מודולי בדיקה, ו-173 נתיבים דמויי קבצים (כולל 50 font edges) שדורשים אימות מול source inventory ו-runtime consumer.
- בדיקת `xtask/src/main.rs` העלתה סיבה מבנית: `extract_quoted_strings` סורק את כל מחרוזות JavaScript ללא lexer או הקשר תחבירי, ו-`is_path_candidate` מקבל סיומות bare. לכן אין לפרש 413 unresolved edges כ-413 נכסים חסרים.
- נוצר דו״ח מפורט: [asset-edge-triage-41355862823.md](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/blob/main/docs/provenance/asset-edge-triage-41355862823.md). זה audit ראשוני בלבד; לא שיניתי scanner, manifest, baseline או סיווג edge.

### החלטת המשך

- לשמר את ה-manifest הקיים כראיית baseline.
- לתכנן תיקון extractor שמבוסס על תחבירי import/require/new URL ו-schema ייעודי ל-font catalogs; לא לסרוק כל quoted string ולא להסתמך על regex נוסף בלבד.
- להוסיף fixtures של false-positive ושל positive references לפני שינוי scanner, ואז להשוות manifest חדש byte-for-byte ולבצע edge diff.
- אין לסגור provenance/manifest עד שה-rerun 41360202733 יסתיים, ה-artifact החדש ייבדק, וכל unresolved שנותר יסווג עם owner והסבר.


### אימות חוזר של manifest — 2026-11-02

- [Manifest scan 41360202733](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733) הסתיים ב-success והעלה artifact [13105540442](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733/artifacts/13105540442).
- ה-manifest החוזר הוא 70,097,869 bytes, SHA-256 `304cd6fe197726cf21a8fe2a7ba1e4b38d50e9aef2ae6da8a7fbe3ec5f8c8c47`, זהה byte-for-byte ל-artifact 13103227452 מהסריקה הקודמת.
- מכאן שהסריקה דטרמיניסטית, אך 413 unresolved edges אינם נפתרים. דו״ח triage עודכן עם הראיה החוזרת. הסיבה המבנית ל-false positives עדיין מחייבת תיקון extractor; אין לסגור משימות provenance/scanner.
- ריצה נוספת 41360675212 הופעלה בעקבות עדכון מסמכי provenance ונשארה פעילה בזמן הבדיקה; אינה נדרשת כדי לאמת את ה-byte-identical result שכבר הושג.


## עדכון ביצוע — 2026-11-02, תיקון extractor והרחבת חלון full-corpus

### תיקון Legacy runner timeout

- [Legacy runner 41356048158](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41356048158) הגיע ל-full-corpus אחרי smoke מוצלח, אך GitHub ביטל את ה-job לאחר 60 דקות בדיוק. זו הייתה מגבלת timeout-minutes: 60 ב-workflow, לא ביטול עקב push.
- שיניתי את .github/workflows/legacy-runner.yml ל-timeout-minutes: 360 (המקסימום ל-GitHub-hosted runner) ב-commit 4cb14433782d159d9f2fd24c6511af3a6370af8d.
- [Legacy runner 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075), שהופעל על commit זה, עבר שוב את ה-checkout, התקנת Node/dependencies/Chromium ואת smoke; full-corpus comparison עדיין in progress בזמן כתיבת העדכון. אין עדיין report או artifact מלא. אין לבטל את הריצה.

### תיקון ראשוני ל-JavaScript asset extractor

- הוחלפה סריקת כל quoted strings ב-JavaScript ב-tokenizer לקסיקלי מוגבל: הערות מדולגות, מחרוזות ו-template literals נשמרים כיחידות נפרדות, ונתיבי נכסים מזוהים רק בהקשרים מוגדרים.
- נתמכים static fetch(...), require(...), import (כולל side-effect import), from, new URL(...), ושדות object מפורשים file/src/href/poster/url/path/asset/fontFile.
- מערכי שמות סטטיים שמוגדרים בשמות כמו PRIORITY_FONT_FILES נשמרים, כדי לא לאבד font basenames שמשמשים ליצירת URLs דינמיים.
- HTML attributes ו-CSS url(...) נבדקים בתוך string/template content; dynamic template expressions אינם מסווגים כנתיבים סטטיים.
- is_path_candidate ו-normalize_asset_reference דוחים extension-only fragments ו-directory-only values כגון .json, scripts/.css, fonts/ ו-assets/.
- נוספו unit tests ל-imports, require, new URL, fetch, asset object fields, font inventories, comments, template markup וערכים שנדחים.
- קוד: [xtask/src/main.rs](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/blob/main/xtask/src/main.rs), commits 92b09e410d98473502be531992a35f9d7100709d ו-028319268d56eb2492ed36a16d861f02c2bcfcc7 (rustfmt בלבד ב-commit השני).

### אימות קוד

- [Rust CI 41361439397](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361439397) הסתיים ב-success על commit 028319268d56eb2492ed36a16d861f02c2bcfcc7.
- כל חמשת ה-jobs עברו: Quality Ubuntu/macOS/Windows, Fixed-point scale reproducibility ו-negative unwrap quality gate.
- בשלוש הפלטפורמות עברו Formatting, Clippy, unit/integration tests, doc tests, release build ו-repeat clean build + workspace metadata comparison. לכן בדיקות ה-unit החדשות ל-extractor עברו בכל שלוש הפלטפורמות.
- [Manifest scan 41361439336](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361439336) הופעל על אותו commit ונשאר in progress בזמן העדכון. יש לבדוק את artifact ואת edge diff לאחר סיום; אין להסיק מראש כמה edges הוסרו או נשארו.

### מצב קבלה

- הוכח: Rust CI רב-פלטפורמי ירוק על קוד extractor החדש; timeout של full-corpus הוגדל; smoke runner עובר בריצה החדשה.
- ממתין: manifest scan עדכני, edge diff, סיווג unresolved אמיתיים/false positives, ו-full-corpus report מול baseline.
- אין לשנות baselines או tolerances, אין לסמן unresolved כ-resolved באופן גורף, ואין להתחיל שלב ב. V21 ו-V23 נשארים פרופילים נפרדים.


### Forensic edge diff and extractor correction — 2026-11-02

- [Manifest scan 41361439336](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361439336) הסתיים ב-success והפיק artifact [13105032721](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361439336/artifacts/13105032721). השוויתי אותו ל-[41360202733](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733), כולל כל 18,854 מזהי הרשומות: לא נוספו או הוסרו records, ולא השתנו source hashes.
- בדיפ הראשוני: 86 records שינו edges/status; 486 edges הוסרו ו-2 נוספו. סך ה-asset dependencies ירד מ-41,192 ל-40,708; resolved ירד מ-35,356 ל-35,168; unresolved ירד מ-413 ל-157. אין לראות בירידה הזו לבדה סגירת provenance.
- בבדיקה ידנית של ה-diff התגלה false negative חדש: normalizer דחה whitespace גם בתוך CSS url("Arial Italic.woff2") חוקי. כתוצאה מכך 174 references של שמות קובצי גופן עם רווחים (35 שמות ייחודיים) נעלמו מה-manifest על פני הפרופילים. בנוסף, imports סטטיים ללא סיומת, כגון require('./lib/harness'), סווגו unresolved אף שקיים קובץ .js יחיד.
- תיקנתי את שני המקרים ב-[commit 56ab319](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/56ab3190cb2817864533eb5c536f55640f2a5f4d): whitespace מותר רק במסלול CSS url() מפורש; מחרוזות JS כלליות ממשיכות להידחות כשיש whitespace. נוספה גם resolution לסיומות .js/.mjs/.cjs רק כשיש התאמה יחידה באותו profile, ובדיקות unit לשני המקרים.
- ה-CI של 56ab319 נכשל ב-Formatting בגלל שגיאת תחביר שנוצרה בעריכת guard; הכשל זוהה בלוגים ותוקן ב-[commit 4d4a5c5](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/4d4a5c557289f66353aa5eb482e0b2754cf7553c). אין להסתמך על 56ab319 כתיקון מאומת.
- [Rust CI 41363057774](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363057774) ו-[Manifest scan 41363057769](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363057769) הופעלו על 4d4a5c5 ונמצאים בבדיקה. יש להשתמש רק בתוצאות וב-artifact של הריצה העדכנית להכרעה.
- [Legacy runner 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) עדיין ב-full-corpus comparison; smoke עבר. אין עדיין report/artifacts מלאים, ואין לבטל את הריצה.
- מסקנה: התיקון צמצם false positives משמעותית, אך edge diff הוכיח שחייבים לשמר גם CSS paths עם רווחים ו-module resolution תקני. משימות provenance/manifest/runner נשארות פתוחות; אין לשנות baselines או tolerances ואין להתחיל שלב ב.


### תיקון מבני נוסף ל-extractor — 2026-11-02

- בדיקת [CI 41363057774](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363057774) חשפה שבעריכת guard נשאר אחרי מודול הבדיקות קטע קוד משוכפל ולא שלם, ולכן גם commit 4d4a5c5 לא היה תקין. אין להתייחס לכשל זה ככשל לוגי של ה-extractor; זהו כשל מבני בקובץ המקור.
- ב-[commit 99e4266](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/99e42666f5a6b1fb326b33ac0cab2062e1cea986) הוסר ה-tail המשוכפל. בדיקת מקור לאחר התיקון מראה 1,848 שורות, הגדרה אחת של resolve_local_asset_path ומודול tests אחד. ה-guard התקין, CSS URL עם רווחים, resolver ל-import extensionless ושתי בדיקות היחידה נשמרו.
- [Rust CI 41363244229](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363244229) ו-[Manifest scan 41363244210](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363244210) הופעלו על 99e4266; בעת עדכון זה הם queued/pending. אין עדיין אישור קומפילציה או artifact סריקה.
- הריצות 41363221442/41363221527 על 2d2bede הוחלפו בריצות commit 99e4266; אין להשתמש בהן כראיית קבלה.
- Legacy runner 41361263075 עדיין פעיל ב-full-corpus; smoke עבר ואין עדיין report/artifacts מלאים.
- שלב א נשאר פתוח. אין להתחיל שלב ב עד CI ירוק, manifest diff מתוקן ו-full-corpus report מול baseline.


### השלמת formatting של extractor — 2026-11-02

- [Rust CI 41363244229](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363244229) עבר את שלבי ה-parser/compile עד שער Formatting, ונכשל רק משום ש-rustfmt דרש לפרוס assert_eq אחד למספר שורות. לא זוהה כשל לוגי בבדיקות, אך ה-CI אינו ירוק.
- תיקון formatting בלבד ב-[commit 17c68ea](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/17c68ea72860798d5f1f31b2c02f9acca7a06a22). לא שונו תנאי resolver או התנהגות scanner.
- [Rust CI 41363321746](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363321746) ו-[Manifest scan 41363321758](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363321758) הופעלו על 17c68ea והם queued/pending. אלה הריצות העדכניות לבדיקת הקוד.
- [Manifest scan 41363244210](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363244210) על 99e4266 עדיין in progress בעת הבדיקה, אך אינו הריצה הקובעת לאחר commit formatting החדש.


### סיווג מלא של unresolved במניפסט — 2026-11-02

- [Manifest scan 41363321758](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363321758) הסתיים בהצלחה וה-artifact [13106386301](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363321758/artifacts/13106386301) הושווה ישירות ל-[baseline manifest 41360202733](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733), artifact [13105540442](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733/artifacts/13105540442).
- אימות זהות: 18,854 compatIds זהים; 0 sourceSha256 השתנו. סיכום: 41,192→40,881 asset edges; 35,356→35,359 resolved; 413→139 unresolved. בדיפ: 313 edges הוסרו ו-2 נוספו; resolved: 18 נוספו ו-15 הוסרו; unresolved: 274 הוסרו.
- נבדקו כל 139 unresolved על פני 17 רשומות. חלוקה: 133 הפניות לארכיונים היסטוריים תחת archive/v19, v20.x ו-v21 שאינם קיימים בעץ ה-pinned של IMUD_TORANI; 3 הפניות לסקריפטים engine-diagnostics.js, engine-inputs.js, engine-trace.js שמוזכרים ב-Gold renderer.html אך אינם קיימים בעץ Gold; 2 הפניות cli.js מתוך package-lock.json (bin paths של חבילות npm, לא קבצים מקומיים); הפניה אחת tools/playwright/package.json שנובעת מ-require('playwright/package.json') — bare module specifier של npm, לא נתיב מקומי.
- לפיכך 136 unresolved הם הפניות אמיתיות לקבצים חסרים/ארכיון חסר לפי העצים הנעולים, ו-3 הם false positives של extractor. לא סיווגתי את 133 הפניות הארכיון כ-resolved ולא הסתרתי אותן: נדרש להחליט אם הארכיונים אמורים להיכלל ב-snapshot המקור או להישאר unresolved היסטוריים.
- בדיקת עץ המקור בוצעה מול commits הנעולים במניפסט: IMUD_TORANI `6dd79d0...`, Gold `a444d34...`, Orange `0b0ee27...`. Gold renderer מצביע בפועל על שלושת קובצי engine החסרים; אין להוסיף להם synthetic placeholders.
- [Rust CI 41363362478](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41363362478) עבר בכל חמשת jobs על ראש המאגר שהיה אז: Quality Ubuntu/macOS/Windows, fixed-point reproducibility ו-negative unwrap gate.
- תיקון false-positive נוסף ב-[commit 1a1ce26](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/1a1ce26b6fefa3db40decc304b86353284e466d6): bare Node module specifiers אינם נחשבים local paths; package-lock.json אינו נסרק כאילו ערכי bin פנימיים של npm הם קבצי repo. נוספו שתי בדיקות יחידה. [CI 41367799248](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41367799248) ו-[Manifest scan 41367799385](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41367799385) queued בעת הרישום; אין לטעון שהתיקון אומת עד לסיום.
- [Legacy runner 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) עדיין ב-full-corpus comparison; smoke עבר, אין report מלא.
- מסקנה: extractor המתוקן החזיר הפניות CSS עם רווחים, פתר imports extensionless חד-משמעיים, וכעת מתקן גם false positives של npm. נותרו 136 unresolved אמיתיים לפי snapshot נעול; יש להכריע במדיניות archive ולהשלים את שלושת הסקריפטים החסרים ב-Gold או לתעד את חסרונם במפורש. שלב א נשאר פתוח.


### תיקון Formatting לאחר בדיקת npm false positives — 2026-11-02

- [CI 41367799248](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41367799248) נכשל בשלושת שערי Quality רק בשלב rustfmt; fixed-point ו-negative unwrap jobs עברו. לא היה כשל קומפילציה/לוגיקה שדווח.
- לוג rustfmt הצביע על שני אזורים בלבד ב-`xtask/src/main.rs`: תנאי הסינון של package-lock ושתי assert-ים בבדיקה החדשה. תיקון formatting בלבד ב-[commit 4af38d4](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/4af38d4779c8624924af1063e916246bcd71dab3).
- [Rust CI 41367871354](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41367871354) ו-[Manifest scan 41367871327](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41367871327) queued על 4af38d4; אלו הריצות הקובעות לתיקון npm false positives.
- Manifest 41367799385 על 1a1ce26 עדיין in progress אך הוחלף על ידי הסריקה החדשה לאחר תיקון formatting.
- Legacy runner 41361263075 עדיין full-corpus in progress; אין report.


### המשך תיקון extractor ואימות CI — 2026-11-02

- סריקת manifest [41367871327](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41367871327) על commit 4af38d4 הסתיימה בהצלחה. artifact [13108185810](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41367871327/artifacts/13108185810) הושווה ל-baseline artifact [13105540442](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733/artifacts/13105540442): 18,854 compatIds זהים ו-0 שינויי sourceSha256. ספירות: assetDependencies 41,192→40,626; resolved 35,356→35,221; unresolved 413→35. edge diff: assetDependencies +1/-567; resolved +18/-153; unresolved +0/-378.
- פירוט 35 unresolved ב-artifact זה: 32 הפניות לקבצים היסטוריים חסרים תחת archive/v19, archive/v20.1 ו-archive/v21; 3 סקריפטי Gold חסרים engine-diagnostics.js, engine-inputs.js, engine-trace.js. אין לסמן אותם כ-resolved בלי החלטת מקור/בעלים.
- התברר ש-artifact של 4af38d עדיין אינו מספיק לקבלת extractor: בדיקת יחידה הראתה שהשימוש ב-normalize_module_reference במערכי font inventory משמיט basenames תקינים כגון Alpha-Black.woff2; וכן החרגת package-lock.json כולה השמיטה resolved URLs חיצוניים, אף שנדרש רק לדחות npm bin paths כגון cli.js.
- Rust CI [41369416363](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369416363) על 97ab8c9: Formatting ו-Clippy עברו ב-Linux, אך 2 מתוך 21 unit tests נכשלו (font inventory ו-package-lock external resolved). fixed-point ו-negative unwrap עברו. התיקון ב-[7bcd686](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/7bcd6862eb2ebb4c8a8b8d53578d655fb716bac6): font inventory משתמש ב-normalize_asset_reference; package-lock סורק רק ערכי מפתח resolved באמצעות tokenizer, בלי להחזיר bin path extraction.
- CI [41369485795](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369485795) על 7bcd686 נכשל בשער rustfmt בלבד (פריסת pattern ב-helper החדש). תיקון formatting בלבד ב-[218d50a](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/218d50abc1758fa9b187f3d39a9882d876ee7321).
- ריצות האימות העדכניות על 218d50a: [Rust CI 41369525393](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369525393) queued ו-[Manifest 41369525487](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369525487) pending בעת הרישום. יש לבדוק את שתיהן; אין לייחס להן הצלחה עדיין.
- [Legacy runner 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) נשאר ב-full-corpus comparison; smoke עבר, שלב השוואת הקורפוס עדיין in_progress, אין report/artifact מלא. אין לבטל.
- שלב א נשאר פתוח. אין לשנות baselines/tolerances, אין לסמן provenance או manifest כ-complete, ואין להתחיל שלב ב.


### תיקון מסנן fetch והמשך אימות — 2026-11-02

- [Rust CI 41369525393](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369525393) על 218d50a: rustfmt ו-Clippy עברו בכל שלוש הפלטפורמות, אך unit test אחד נכשל בכולן. 20/21 בדיקות עברו; הכשל היחיד היה שהפניה סטטית fetch("fonts/fonts.json") לא נכללה, משום ש-fetch השתמש בטעות ב-module resolver המחייב ./ או ../.
- בדיקת package-lock_bin_paths עברה לאחר התיקון הקודם; בדיקת font inventory עברה. מכאן שהטיפול בשני הכשלים הקודמים הצליח, ונשארה הבחנה בין fetch asset URL לבין require/import module specifier.
- תיקון ב-[commit 73557d4](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/73557d41d240698c847e5e4d88fafcf82235e4d3): fetch משתמש ב-normalize_asset_reference; require/import ממשיכים להשתמש ב-normalize_module_reference. לא שונתה מדיניות bare module specifier.
- ריצות על 73557d4: [Rust CI 41369621550](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369621550) ו-[Manifest 41369621509](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369621509) pending בעת העדכון. יש לבדוק אותן לפני קבלת השינוי.
- Manifest [41369525487](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369525487) על 218d50a עדיין in_progress; הוא קודם לתיקון fetch ולכן אינו קובע למניפסט הסופי.
- [Legacy runner 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) עדיין in_progress ב-full-corpus comparison; smoke עבר ואין report מלא.
- Stage A נשאר פתוח. אין לשנות baselines/tolerances, אין לסגור provenance/manifest, ואין להתחיל Stage B.


### קבלת CI מלאה לאחר תיקון fetch — 2026-11-02

- [Rust CI 41369673903](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369673903) הסתיים success על ראש המאגר 5f11938 (קוד extractor 73557d4): כל חמשת ה-jobs עברו. Quality Linux/macOS/Windows עברו Formatting, Clippy, 21 unit tests, documentation tests, release build ו-repeat clean build/metadata comparison; fixed-point reproducibility ו-negative unwrap gate עברו.
- [Manifest scan 41369621509](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369621509) על קוד 73557d4 עדיין in_progress בשלב יצירת manifest canonical פעמיים. אין artifact חדש עדיין; יש להוריד ולהשוות ל-baseline 41360202733 לאחר השלמתו.
- [Legacy runner 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) עדיין ב-full-book corpus comparison אחרי smoke מוצלח; כ-99 דקות מאז תחילת הריצה בזמן בדיקה זו. timeout 360 דקות. אין לבטל ואין לטעון לתוצאת corpus עד קבלת report/artifacts.
- הקוד האחרון עבר CI מלא, אך manifest/provenance עדיין לא מתקבלים: נדרשים edge diff של 73557d4, הכרעה על 32 archive references ושלושת Gold engine scripts, ו-full corpus report. Stage A פתוח; אין Stage B.


### Forensic manifest review + הרחבת extractor — 2026-11-02

- [Manifest 41369621509](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369621509), artifact [13109210869](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41369621509/artifacts/13109210869), נבדק מול baseline artifact [13105540442](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733). 18,854/18,854 compatIds זהים; 0 sourceSha256 changes. Baseline→new: asset edges 41,192→40,877 (317 removed, 2 added); resolved 35,356→35,359 (15 removed, 18 added); unresolved 413→136 (277 removed, 0 added).
- 136 unresolved ב-artifact 73557d4: 132 edges (121 unique paths) תחת שבעה תתי-עצים היסטוריים ב-archive של IMUD; 3 קובצי Gold חסרים מ-renderer.html (engine-diagnostics.js, engine-inputs.js, engine-trace.js); והפניה אחת tests/features/default_settings.json שנבעה מהנחת שגויה ש-fetch יחסי מתוך test JS נפתר ביחס לתיקיית קובץ ה-JS. אין להסיק מכך שכל תיקיות archive חסרות: אלה נתיבי תלות ספציפיים, וקיום כל אחד מהם צריך להיבדק בנפרד.
- 277 unresolved edges שהוסרו: 118 extension-only fragments, 62 markup/template fragments, 14 host-specific absolute paths, 10 npm script command strings, 4 npm package bin false positives, ו-59 אחרים שדורשים review. בנוסף 15 resolved edges נעלמו (חלקם הפניות אמיתיות), ולכן artifact 73557d4 אינו מספיק לאישור extractor. 18 resolved edges חדשים הם בעיקר imports יחסיים extensionless שנפתרו באופן חד-משמעי.
- תיקוני extractor נוספים ב-[8b0f17d](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/8b0f17d0c56ed955494c5ec56362286dc2ae59f6), [e9c7410](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/e9c74108e1c033efa7b8d8bc892d3742c4e358a8), [8ce3bd1](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/8ce3bd16ad88aefcddb2ccef11f8f966347ab1ab), [d28927a](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/d28927adefe555e9e7e1d9348e3b89ae4538567c) ו-[3b4268d](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/3b4268d0e3e806729248aef7bbe0b2d95d1293a2): fetch base semantics for browser tests, DOM href/src/poster assignments, nested template-literal skipping, static path.join(__dirname/root/SCRIPTS_DIR), Playwright page.goto static paths, and readFileSync static fallbacks. נוספו unit tests ייעודיים. שני ניסיונות ביניים נכשלו בשער rustfmt בלבד; התיקונים formatting-only, והקוד העדכני עבר CI מלא.
- [Rust CI 41374863123](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41374863123) על 3b4268d עבר success בכל חמשת ה-jobs, כולל Linux/macOS/Windows, formatting, Clippy, unit/integration/doc tests, release builds, repeat clean build, fixed-point reproducibility ו-negative unwrap gate.
- [Manifest scan 41374863178](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41374863178) על 3b4268d עדיין in_progress; נדרש artifact חדש והשוואה מלאה ל-baseline לפני קבלת התיקונים. יש לוודא שההפניות התקינות שהוחמצו ב-73557d4 חזרו ושלא נוספו false positives.
- [Legacy full-corpus runner 41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) נשאר in_progress בשלב full-book corpus comparison אחרי smoke מוצלח. timeout מוגדר 360 דקות; אין לבטל ואין לטעון לתוצאת corpus עד report/artifacts.
- Stage A נשאר פתוח; tasks 1–2 בלבד סגורות ו-3–10 פתוחות. אין שינוי baselines/tolerances, אין סגירת provenance/manifest, ואין Stage B.


### Manifest re-scan and edge triage — 2026-11-02 (run 41376496462)

- Current code commit at scan time: [526d6a0](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/commit/526d6a04508be4145b09efaf25424ca59fb7ef51). Rust CI [41376496424](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41376496424) passed; manifest workflow [41376496462](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41376496462) passed and generated artifact [13111866073](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41376496462/artifacts/13111866073).
- Exact comparison to baseline artifact 13105540442: 18,854/18,854 compatIds identical; 0 sourceSha256 changes; asset edges 41,192→40,940 (-252); resolved edges 35,356→35,387 (+31); unresolved edges 413→161 (-252). Edge diff: 278 removed, 26 added; 0 previously resolved edges lost, 31 newly resolved; 263 previous unresolved removed, 11 new unresolved. The 11 new unresolved are verified static fallback paths missing at their pinned paths, not suppressed as noise.
- Current 161 unresolved edges: IMUD V20x 147 edges / 133 unique missing paths across seven partial archive subtrees; Gold V21 13 edges / 4 unique paths (three missing engine scripts plus root-level very_long_test_text.txt fallback referenced by ten scripts); Orange V23 one root-script fallback path escaping the repository. Details and per-subtree counts are in [asset-edge-triage-41376496462.md](docs/provenance/asset-edge-triage-41376496462.md).
- The IMUD archive finding is specifically about 133 referenced files absent from the pinned tree; it does not mean the seven archive directories are absent. Owner decision remains: restore exact historical assets or formally classify the snapshots as partial/non-runnable. Gold/Orange fallback paths also require owner disposition. Do not add suppressions or alter baseline/tolerance.
- No current unresolved edge was demonstrated to be an extractor false positive. Manifest generation and identity/hash stability passed, but provenance/manifest acceptance remains open pending source-owner decisions.
- Legacy full-corpus run [41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) remains in progress at full-book corpus comparison. Smoke, renderer, DOM geometry, and ledger steps completed; no full-corpus report/artifacts yet. Leave it running; do not infer a result.
- Stage A remains open: tasks 1–2 closed, 3–10 open. Stage B not started.


### Stage A task 3 — provenance pin verification — 2026-11-02

- Re-fetched all six commit objects and recursive root trees recorded in `docs/provenance/legacy-source-pins.json`. All six commit objects exist and all six actual root-tree SHAs match the recorded tree SHAs exactly.
- Cross-checked every pinned source file against the corresponding Git tree: **47/47 Git blob SHA-1 values match**, with zero missing paths and zero mismatches (Orange 15, Green 7, Gold 7, Yellow 7, IMUD 6, Typesetter_doc 5).
- Verified the three lineage reference commits cited by Orange provenance are retrievable: Green import point `0ddaf89010e84e9e999c0e1faef2888994393915`, Gold reference point `31ed2ca830c1868cb2c9157d080f8922d112ff8e`, and common-ancestor reference `5c646c1fc2ea52233be76618463933dbe013cf2c`. Their role/relationship is recorded in `docs/provenance/source-of-truth-matrix.md`; existence checks alone are not presented as a full patch-level port audit.
- Re-hashed the five files uploaded with this continuation. Their byte lengths and SHA-256 values match all five `unattributedUserAttachments` entries exactly. They remain an independent, unattributed source set; no branch/repository was guessed.
- Task 3 acceptance criterion (exact reproducible pins and recorded Orange–Green–Gold lineage) is satisfied; task 3 is now checked complete in the 100-task plan. Tasks 1–3 are closed. Tasks 4–10 remain open; Stage B is not started.


### Task 4 partial — pinned default-settings conflict scan — 2026-11-02

- Compared the root `default_settings.json` from Orange, Green, Gold, Yellow and IMUD at the exact pinned commits.
- Counts: 254 / 248 / 264 / 246 / 264 keys; union 267; intersection 246.
- There are 21 key-presence differences and **zero conflicting JSON values/types among keys present in multiple profiles**.
- Presence-only groups: Green-only `allow-terminal-residual-fallback=true` and `balance-chapter-end="balance"`; Orange-only `hdr-h1-title-mode="full"`; Gold+IMUD-only 11 `aq-*` keys; Orange+Gold+IMUD 7 ornament/divider keys.
- Missing is not false/zero/empty and must remain distinct in the future settings model. Gold/IMUD default-settings blob identity does not unify their engine behavior.
- Detailed table and interpretation rule: `docs/provenance/source-of-truth-matrix.md` §10. Task 4 remains open pending the equivalent inventory for presets, assets, tests and remaining source classes.


### Task 4 partial — test harness ownership — 2026-11-02

- Pinned profile test layouts are structurally different: Orange 33 paths under tests/fixtures and an npm test script running seven suites; Green 12 fixtures and no npm test script; Gold 44 test/debug/audit-named paths across scripts/workflows/docs with its own baseline/quick/determinism scripts; Yellow 13 test paths and no root package.json; IMUD 18 test paths and its own quick layout + feature suites.
- Orange/Green share eight test fixture paths: seven have identical blob IDs; one `ornament-geometry-settings.json` differs in six header type values (Orange `ext________2_svg`, Green `none`).
- This confirms test ownership and expected results must remain profile-specific. Full detail in `docs/provenance/source-of-truth-matrix.md` §12.
- Task 4 remains open; counts are structural inventory only, not proof of coverage. Every required compatibility behavior still needs explicit owner and test mapping.


### Latest workflow state — 2026-11-02 03:02 UTC

- Latest documentation/main commit: `1a0cb91d0df73c22a52be8e0f23f37cf27a3f955`.
- Code under validation remains `526d6a04508be4145b09efaf25424ca59fb7ef51`; its dedicated Rust CI [41376496424](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41376496424) passed.
- A docs-triggered Rust CI rerun [41380773019](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41380773019) is in progress. A docs-triggered manifest scan [41380737375](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41380737375) is also in progress; because source code and pinned trees are unchanged, its artifact can serve as a second-run reproducibility comparison against artifact 13111866073 once available.
- Legacy full-corpus run [41361263075](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41361263075) remains in progress at the full-book corpus comparison step. The runner's 360-minute timeout remains in force. Do not wait idle, cancel, or infer its final result.


### Task 4 progress — style-set and preset provenance — 2026-11-02

- Re-fetched the five pinned repository trees and compared root `setting_sets/` files by Git blob SHA-1. All five profiles contain the same three style-set files, byte-identical across profiles: `פרשנות` (`2f92a70`), `קונטרס` (`c56c30f`), and `תלמוד` (`bb99eb2`), each with 249 keys.
- The three shared style sets have identical key sets but distinct values: `קונטרס` and `תלמוד` each differ from `פרשנות` in 16 values. This is shared data with three semantic variants, not a single interchangeable preset.
- IMUD's nested `presets/makom-bina-v20.5/` contains four 249-key snapshots. Compared with the root 264-key default, they each omit 24 root keys, add 9 different keys, and change 66/79/80/82 shared values respectively.
- IMUD's pinned `settings-manager.js` loads root `default_settings.json` and only the three root `setting_sets/` files; it does not reference the nested V20.5 preset folder. The nested files are therefore recorded as separate snapshots, not presumed active UI defaults.
- Archived IMUD V19 and V20.4 defaults have 20 and 38 value differences from the current root default, respectively, plus their own key-presence differences. They remain historical inputs.
- Full blob IDs, counts and interpretation are in `docs/provenance/source-of-truth-matrix.md` §13. Task 4 remains open; this closes only the root style-set/preset comparison slice.
