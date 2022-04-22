pub fn get_value(config: &str, key: &str) -> String {
    let mut found_value = false;
    for token in config.trim().split("=") {
        if found_value {
            return token.to_string();
        }
        if token.starts_with("#") {
            continue;
        }
        if token == key {
           found_value = true;
        }
    }
    return "".to_string();
}
