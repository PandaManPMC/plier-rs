
/// is_blank "" == s.trim()
pub fn is_blank(s: String) -> bool {
    if "" == s.trim() {
        return true;
    }
    false
}

/// is_not_blank "" != s.trim()
pub fn is_not_blank(s: String) -> bool {
    if "" != s.trim() {
        return true;
    }
    false
}