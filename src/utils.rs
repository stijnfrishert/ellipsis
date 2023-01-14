pub fn sanitize(str: &str) -> String {
    if str.chars().all(|c| c.is_alphanumeric() || c == '_') {
        str.to_string()
    } else {
        format!("\"{str}\"")
    }
}
