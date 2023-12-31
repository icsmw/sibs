pub fn trim(src: &str) -> String {
    src.split('\n')
        .map(|s| s.trim())
        .collect::<Vec<&str>>()
        .join("")
}
