pub(crate) fn make_escaped_string(s: &str) -> String {
    let mut r = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '|' => r.push_str("\\|"),
            '\\' => r.push_str("\\\\"),
            '\r' => r.push_str("\\r"),
            '\n' => r.push_str("\\n"),
            '\t' => r.push_str("\\t"),
            _ => r.push(c),
        }
    }
    r
}
