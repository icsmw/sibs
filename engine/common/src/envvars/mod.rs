pub fn is_proptest_debug() -> bool {
    std::env::var("SIBS_DEBUG_PROPTEST")
        .map(|v| {
            let v = v.to_lowercase();
            v == "true" || v == "on" || v == "1"
        })
        .unwrap_or(false)
}
