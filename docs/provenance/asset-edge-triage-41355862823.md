# Audit ראשוני של unresolved asset edges — Manifest 41355862823

תאריך: 2026-11-02  
מאגר יעד: `SUGINACH/Tora_Book_Typesetter_Rust`  
סריקה: [Manifest scan 41355862823](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862823)  
Artifact: [legacy-compatibility-manifest.zip](https://github.com/SUGINACH/Tora_Book_Typesetter_Rust/actions/runs/41355862823/artifacts/13103227452)  
קוד הסורק שנבדק: `xtask/src/main.rs` ב-commit `dd4d33e172f816bdf2fdbb8e70ab53195821a2e2`.

## 1. היקף ונתונים

- 18,854 רשומות במניפסט.
- 413 unresolved local edges, מתוך 270 ערכי edge ייחודיים.
- ה-edges מופיעים ב-64 רשומות מקור.
- חלוקה לפי מאגר מקור:
  - IMUD_TORANI: 271
  - Orange V23: 61
  - Yellow V10: 48
  - Green V23: 23
  - Gold V21: 10
- ברמת הרשומות שמכילות את ה-edges: 290 edges נמצאים ברשומות שסטטוסן `unresolved-local`, ו-123 edges נמצאים ברשומות שסטטוסן `contains-external`. זהו סטטוס ברמת הרשומה; הוא אינו אומר שכל edge ברשומה הוא external.

## 2. ממצא בקוד הסורק

ב-`extract_asset_dependencies` (בערך שורות 547–588) הסורק:
1. מחלץ URL functions ו-HTML attributes.
2. עבור JavaScript, settings ו-data, עובר בנוסף על **כל** מחרוזת במירכאות באמצעות `extract_quoted_strings`.
3. כל מחרוזת שמכילה prefix או סיומת שנראים כמו נתיב/סיומת קובץ מועברת ל-`normalize_asset_reference`.

`extract_quoted_strings` (בערך שורות 595–618) אינו parser של JavaScript ואינו מנהל מצב של template literals/backticks או הקשר תחבירי של המחרוזת. לכן הוא יכול לפרש חלקי markup, דוגמאות, פקודות shell וקטעי מחרוזות בתוך template literal כתלות asset. `is_path_candidate` (בערך שורות 690–728) מקבל גם מחרוזות שמסתיימות בסיומת מוכרת, אפילו אם הערך הוא רק `.json` או `.svg`. בנוסף, נתיב תיקייה כמו `fonts/` עלול להתנרמל ל-`local:fonts`.

זהו פגם דיוק מוכח באסטרטגיית החילוץ, לא הוכחה שכל 413 ה-edges שגויים. אין למחוק או להמיר edges לפני בדיקת המקור והקשרם.

## 3. חלוקה היוריסטית ראשונית

החלוקה הבאה היא כלי triage בלבד; אינה תיוג סמנטי סופי:

| מחלקת triage | מספר edges | פרשנות ראשונית |
|---|---:|---|
| סיומת בלבד כגון `.json`, `.svg`, `scripts/.css` | 118 | מועמד חזק ל-false positive של חילוץ מחרוזות |
| קטעי inline markup / SVG / HTML | 62 | מועמד חזק ל-false positive; יש לאמת מול שורת המקור |
| directory token כגון `fonts` | 7 | כנראה directory ולא קובץ asset |
| נתיבי הרצה סביבתיים כגון Chrome או Chromium | 9 | נתיב executable של סביבת בדיקה, לא בהכרח asset של הפרופיל |
| פקודות/מודולי בדיקה כגון `node tests/...` ו-`tests/lib/...` | 27 | דורש הקשר; ייתכן שהסורק תפס פקודה או specifier חלקי |
| שמות קובץ שנראים תקינים, כולל fonts | 173 | אין לסווג אוטומטית; דורש בדיקת קיום, base path, profile ו-runtime consumer |
| ערכים אחרים | 17 | דורש בדיקה ידנית |

## 4. דוגמאות קונקרטיות

- `IMUD_TORANI/app.js` מייצר edges כגון `local:, 'preserveAspectRatio=`, `local:.json`, `local:.svg` ו-`local:fonts`. צירוף זה מתאים לחילוץ קטעי קוד/markup, ולא לנתיבי קבצים תקינים.
- `Orange/tests/lib/harness.js` מייצר `tests/lib/.css`, `tests/lib/.ico`, `tests/lib/.jpg`, `tests/lib/.js`, `tests/lib/.json`, `tests/lib/.otf`, `tests/lib/.png` ו-`tests/lib/.svg`. סיומות ריקות אלה אינן שמות קבצים.
- `Yellow/scripts/geometry-audit-headless.mjs` מייצר נתיבים כגון `scripts/.css` ו-`scripts/.woff2`, וכן `Applications/Google Chrome.app/Contents/MacOS/Google Chrome`. יש להפריד בין asset refs סטטיים לבין נתיבי executable וקוד שמרכיב נתיב דינמי.
- מנגד, רשומות IMUD מכילות גם שמות fonts אמיתיים כגון `Alpha-Black.woff2` ו-`FrankReal-Regular.woff2`. אלה חייבים להיבדק מול inventory הגופנים והפרופיל; אין לפטור אותם כ-false positives רק משום ש-edges אחרים באותה רשומה שגויים.

## 5. מסקנות שאפשר לקבוע כעת

1. מנגנון החילוץ הנוכחי אינו מדויק דיו כדי להתייחס לכל unresolved edge כהוכחה לנכס חסר.
2. אין הצדקה להקטין את 413 המספר באמצעות שינוי baseline, התעלמות גורפת מ-JavaScript או המרת unresolved ל-resolved.
3. יש לשמור את manifest המקורי כראיית baseline ולבצע תיקון extractor נפרד, עם fixtures של false positives ושל references אמיתיים.
4. לאחר תיקון הסורק יש להריץ את אותו scan פעמיים, להשוות byte-for-byte, ולבצע diff של כל edge שנוסף/הוסר/שינה resolution.
5. כל unresolved שנותר חייב לקבל סיווג מפורש: missing local file, dynamic/runtime path, external/environment dependency, false positive מוכח, או unresolved requiring owner review.

## 6. תכנון תיקון ובדיקות

- להחליף את סריקת כל quoted strings ב-JavaScript בחילוץ תחבירי מצומצם: static `import`/`export ... from`, `require("...")`, `import("...")`, `new URL("...", import.meta.url)`, וקריאות URL/asset מפורשות שהפרופילים משתמשים בהן.
- אם נדרשת תמיכה במחרוזות font-catalog שאינן import, להוסיף extractor ייעודי לפי schema/מיקום מוכר, ולא לסרוק כל string literal.
- להוסיף lexer מינימלי או parser מתאים שמדלג על template-literal raw text ומזהה expressions; לא לנסות לתקן את הבעיה באמצעות regex נוסף בלבד.
- לדחות path candidates שהם סיומת בלבד, directory-only, shell command או fragment של markup.
- להוסיף unit tests עבור הדוגמאות לעיל, לצד positive tests ל-font basename, CSS `url(...)`, HTML `src/href`, ES module imports ו-`new URL`.
- לא לשנות את `resolve_local_asset_path` או את profile isolation כדי להעלים unresolved אמיתיים.

## 7. מצב

זהו audit ראשוני של ה-artifact הקיים, לא תיקון קוד ולא סגירת משימה 3/7/8. ה-manifest החדש 41360202733 עדיין נדרש להשלמה; לאחר קבלת ה-artifact יש לחזור על אותו triage ולוודא אם התוצאות זהות. Orange V23 ו-Gold V21 נשארים נפרדים; אין למזג פרופילים, fixtures או baselines.
