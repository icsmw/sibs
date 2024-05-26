pub fn get_name(path: &str) -> String {
    let parts = path.split("::").collect::<Vec<&str>>();
    let count = parts.len();
    parts
        .into_iter()
        .skip(count.saturating_sub(2))
        .collect::<Vec<&str>>()
        .join("::")
}

pub fn get_last_name(path: &str) -> String {
    path.split("::")
        .last()
        .expect("Module should have at least one member")
        .to_owned()
}
