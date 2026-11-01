# קובץ מעבר — Tora Book Typesetter Rust

עודכן: 2026-11-02  
מאגר: [SUGINACH/Tora_Book_Typesetter_Rust](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust)  
ענף: `main`  
נקודת בסיס לפני עדכון המעבר: `6a2afeaf49a764bcdb79af63739a3dca524833bb`

## מטרת העבודה

מנוע עימוד תורני ב-Rust, דטרמיניסטי ושקוף, עם מודל גאומטרי מדויק, הפרדה בין semantic/layout/rendering, decision ledger בר-ביקורת וייצוא שאינו תלוי במדידות WebView. תוכנית 100 המשימות היא מסגרת הביצוע המחייבת. בכל הודעת המשך יש להשלים יחידת עבודה אחת מוגדרת, להריץ את הבדיקות הרלוונטיות, לדחוף commit ולעדכן את שלושת מסמכי המעקב.

## מצב כולל

- שלב א (משימות 1–10) פעיל ועדיין פתוח.
- משימות 1–2 הושלמו והוכחו ב-[CI 41347908551](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41347908551).
- אין להתחיל שלב ב עד סגירת משימות 3–10 ושערי הקבלה שלהן.
- אין לטעון שקיים מנוע עימוד עובד: עדיין אין מימוש production של parser, shaping, pagination או renderer.
- V21 ו-V23 נשארים מבודדים לחלוטין. אין למזג קוד, fixtures, baselines או החלטות אלגוריתמיות ביניהם.

## מצב Git ובדיקות אחרונות

- commit אחרון לפני יצירת קובץ זה: `6a2afeaf49a764bcdb79af63739a3dca524833bb`.
- [Rust CI 41355862773](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862773) נפתח אחרי תיקוני syntax ו-rustfmt; היה בתהליך בעת כתיבת העדכון.
- [Manifest scan 41355862823](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862823) נפתח על אותו commit; היה pending/in progress בעת כתיבת העדכון.
- הכשל ב-`e2b8a1d...` נגרם מבלוק `if` לא סגור ב-`normalize_asset_reference`. תוקן ב-`e8aac03...`.
- לאחר מכן rustfmt הצביע על שתי סטיות ב-`xtask/src/main.rs`; תוקנו ב-`6a2afea...`.
- [Manifest scan קודם 41349723140](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41349723140) עבר, אך קדם להרחבת סריקת asset dependencies. לכן הוא אינו סוגר את ה-manifest הנוכחי.
- [Full-corpus legacy run 41349079957](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41349079957) בוטל; אין baseline replay מלא מאושר.
- בקובץ workflow תוקנה מדיניות concurrency: כל ריצה תזוהה לפי commit ו-`cancel-in-progress: false`, כדי ש-push מאוחר לא יבטל full-corpus replay.

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

1. לבדוק את Rust CI 41355862773 ואת manifest scan 41355862823. אם נכשלו, לפתוח logs ולתקן לפני כל פיתוח חדש.
2. לוודא שה-legacy runner הופעל מחדש בעקבות שינוי concurrency; אם לא, להפעיל workflow ידנית. אין לבטל את full-corpus run.
3. לבדוק את Orange V23 full-corpus report מול baseline המוצמד, בלי שינוי tolerance או baseline כדי להשיג pass.
4. לסגור provenance, source-of-truth matrix, scanner, manifest ו-parity matrix רק אחרי ראיות עדכניות.
5. להשלים ניסויי fixed-point, shaping/bidi, Tauri ו-PDF ולתעד במפורש מה עבר ומה עדיין חסר.
6. רק אחרי שכל משימות שלב א נסגרו, להתחיל משימה 11.

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
