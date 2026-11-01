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

