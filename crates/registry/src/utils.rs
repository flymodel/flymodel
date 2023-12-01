pub fn maybe_env(value: String) -> Option<String> {
    if value.len() < 4 {
        return Some(value);
    }

    // if value.starts_with("env:") {
    //     return std::env::var(value.trim_start_matches("env:")).map_err(|e| anyhow::format_err!(e));
    // } else {
    return Some(value);
    // }
}
