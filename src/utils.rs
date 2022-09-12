/// Returns a cross-platform-filename-safe version of any string.
///
/// This is used internally to generate app data directories based on app
/// name/author. App developers can use it for consistency when dealing with
/// file system operations.
///
/// Do not apply this function to full paths, as it will sanitize '/' and '\';
/// it should only be used on directory or file names (i.e. path segments).
pub fn sanitized(component: &str) -> String {
    let mut buf = String::with_capacity(component.len());
    for (i, c) in component.chars().enumerate() {
        let is_alnum = c.is_ascii_alphanumeric();
        let is_space = c == ' ';
        let is_hyphen = c == '-';
        let is_underscore = c == '_';
        let is_period = c == '.' && i != 0; // Disallow accidentally hidden folders
        let is_valid =
            is_alnum || is_space || is_hyphen || is_underscore || is_period;
        if is_valid {
            buf.push(c);
        } else {
            use std::fmt::Write;
            let _ = write!(&mut buf, ",{},", c as u32);
        }
    }
    buf
}
