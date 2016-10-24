/// Returns a cross-platform-filename-safe version of any string.
///
/// This is used internally to generate app data directories based on app
/// name/author. App developers can use it for consistency when dealing with
/// file system operations.
pub fn sanitized(component: &str) -> String {
    let mut buf = String::with_capacity(component.len());
    for c in component.chars() {
        let n = c as u32;
        let is_lower = 'a' as u32 <= n && n <= 'z' as u32;
        let is_upper = 'A' as u32 <= n && n <= 'Z' as u32;
        let is_letter = is_upper || is_lower;
        let is_number = '0' as u32 <= n && n <= '9' as u32;
        let is_space = c == ' ';
        let is_hyphen = c == '-';
        let is_underscore = c == '_';
        let is_valid = is_letter || is_number || is_space || is_hyphen || is_underscore;
        if is_valid {
            buf.push(c);
        } else {
            buf.push_str(&format!(",{},", n));
        }
    }
    buf
}
