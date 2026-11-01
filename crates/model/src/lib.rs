//! מודל המקור והתוכן של tora-typesetter.
//!
//! בשלב זה החבילה היא שלד workspace בלבד. טיפוסי המקור וה-AST יתווספו
//! במשימות המוקדשות למודל הנתונים, תוך שימור SourceSpan וזהויות legacy.

/// מזהה יציב לחבילת המודל, המשמש לבדיקת smoke של ה-workspace.
pub const CRATE_ID: &str = "tora-model";

#[cfg(test)]
mod tests {
    use super::CRATE_ID;

    #[test]
    fn crate_identity_is_stable() {
        assert_eq!(CRATE_ID, "tora-model");
    }
}
