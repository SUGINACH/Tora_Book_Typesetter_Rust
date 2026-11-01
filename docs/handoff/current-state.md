# קובץ מעבר — Tora Book Typesetter Rust

עודכן: 2026-11-02 (לאחר CI ירוק וריצות manifest/full-corpus פעילות)  
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

- תיקון rustfmt נמצא ב-commit `dd4d33e172f816bdf2fdbb8e70ab53195821a2e2`; לאחריו עודכנו מסמכי המעקב ב-commits `6810658c428c77e007d1667ef9c91ba2eff68d74` (worklog), `3152c7430e396bedbf09c99ac338e16b594d1605` (תוכנית 100 המשימות), `a173f38b1779ffe165776c21a7771468fccea135` (handoff) ו-commit התיקון הנוכחי של ledger.
- [Rust CI 41360259748](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360259748) עבר על commit `3c8387f894c3c5bbd429ee98f88f42c9b78364d3`, event מסוג push, conclusion=`success`. זהו commit הקוד האחרון שנבדק; commits התיעוד המאוחרים יותר דורשים CI על HEAD העדכני. כל חמשת ה-jobs עברו: שלושת Quality jobs (Ubuntu/macOS/Windows), fixed-point reproducibility ו-negative unwrap quality gate. בכל שלוש הפלטפורמות עברו Formatting, Clippy, unit/integration tests, doc tests, release build ו-repeat clean build + metadata comparison.
- [Manifest scan 41355862823](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862823) עבר בעבר: 6 sources, 18,854 records, 35,356 resolved local edges, 413 unresolved local edges, 5,423 external references; שתי סריקות byte-identical. [Artifact](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862823/artifacts/13103227452). 413 unresolved edges עדיין לא סווגו סופית.
- [Manifest scan 41360202733](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733), על commit `dd4d33e172f816bdf2fdbb8e70ab53195821a2e2`, הסתיים ב-success. Artifact [13105540442](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733/artifacts/13105540442) מכיל manifest 70,097,869 bytes, SHA-256 `304cd6fe197726cf21a8fe2a7ba1e4b38d50e9aef2ae6da8a7fbe3ec5f8c8c47`, זהה byte-for-byte ל-13103227452. הסריקה דטרמיניסטית, אך 413 unresolved edges נשארו ודורשים triage ותיקון extractor.
- [Legacy runner 41356048158](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41356048158), על target commit `8ccd6611c19a87abf5e7934c9eb6b92eb992362a`: checkout, התקנת Node/dependencies/Chromium ו-offline smoke עברו; Orange V23 full-book corpus מול baseline עדיין in progress. ה-workflow נבדק ומוגדר עם concurrency group ייחודית לכל commit ו-`cancel-in-progress: false`; אין לבטל אותו. אין עדיין report מלא או artifacts.
- [Legacy full-corpus run קודם 41349079957](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41349079957) בוטל בעבר ואינו ראיית baseline replay מלאה.
- תיקון rustfmt לא שינה לוגיקת scanner, נתוני מקור, אלגוריתמי עימוד או baselines.
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

1. להשתמש בתוצאת [Manifest scan 41360202733](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41360202733) שכבר עברה וב-artifact byte-identical. לתקן את JS asset extractor עם lexer/תחביר מוגדר ו-fixtures; אין לסגור provenance או manifest לפני edge diff וסיווג unresolved אמיתיים. ריצה נוספת 41360675212 הופעלה בעקבות עדכון docs/provenance ועדיין נפרדת מהראיה שכבר הושלמה.
2. להמשיך לעקוב אחרי [Legacy runner 41356048158](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41356048158) עד סיום full-book corpus; לבדוק report, DOM, geometry, ledger ו-provenance artifacts מול baseline המוצמד. אין לשנות baseline/tolerance ואין לבטל את הריצה.
3. לבדוק את Rust CI האחרון שנוצר על HEAD הנוכחי של main; עדכון מסמכים מפעיל CI מחדש, ולכן יש לאמת את ה-run שמצביע בדיוק ל-HEAD העדכני ולא להסתמך על run קודם.
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
