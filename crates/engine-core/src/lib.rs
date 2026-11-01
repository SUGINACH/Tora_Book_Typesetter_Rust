//! ליבת מנוע העימוד.
//!
//! החבילה תכיל בהמשך LayoutInput/Output, מועמדים, אילוצים, cost vector,
//! tie-break ו-decision ledger. בשלב האתחול אין בה עדיין החלטות עימוד.

/// מזהה יציב לחבילת הליבה, המשמש לבדיקת smoke של ה-workspace.
pub const CRATE_ID: &str = "tora-engine-core";

#[cfg(test)]
mod tests {
    use super::CRATE_ID;

    #[test]
    fn crate_identity_is_stable() {
        assert_eq!(CRATE_ID, "tora-engine-core");
    }
}
