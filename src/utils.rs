pub fn sanitize(str: &str) -> String {
    let is_valid = match str.chars().next() {
        Some(x) if x.is_numeric() => str.parse::<f32>().is_ok(),
        Some(_) => str.chars().all(|c| c.is_alphanumeric() || c == '_'),
        None => return String::new(),
    };

    if is_valid {
        str.to_string()
    } else {
        format!("\"{str}\"")
    }
}
