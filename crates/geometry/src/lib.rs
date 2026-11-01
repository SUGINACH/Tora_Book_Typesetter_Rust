//! גאומטריה ויחידות מדידה של מנוע העימוד.
//!
//! טיפוס fixed-point וכל פעולות הגאומטריה יתווספו במשימות 21–22.
//! אין להכניס לכאן חישובי floating-point המשפיעים על החלטות העימוד.

/// מזהה יציב לחבילת הגאומטריה, המשמש לבדיקת smoke של ה-workspace.
pub const CRATE_ID: &str = "tora-geometry";

#[cfg(test)]
mod tests {
    use super::CRATE_ID;

    #[test]
    fn crate_identity_is_stable() {
        assert_eq!(CRATE_ID, "tora-geometry");
    }
}
