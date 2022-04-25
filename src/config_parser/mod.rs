//! config parser for minecraft server.properties files

/** accepts a server.properties file as a '&str' and will return a 'Vec<&str>' in a format acceptable by all functions that manipulate it * */
pub fn get_config(config: &str) -> Vec<&str> {
    let configuration: Vec<&str> = config
        .trim()
        .split("\n")
        .filter(|x| !x.starts_with("#"))
        .collect();
    return configuration
}

/// gets a value from a valid config 'Vec<&str>'
/// returns an empty string when a option doesn't have a value
pub fn get_value(config: Vec<&str>, key: &str) -> String {
    for value in config {
        let parsed: Vec<&str> = value.split("=").collect();
        if parsed[0] == key {
            return parsed[1].to_string()
        }
    }
    return "".to_string() // use empty string when error
}
