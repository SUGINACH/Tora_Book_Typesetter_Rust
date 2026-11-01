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
