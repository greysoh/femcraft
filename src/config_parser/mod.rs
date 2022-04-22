pub fn get_config(config: &str) -> Vec<&str> {
    let mut found_value = false;
    let configuration: Vec<&str> = config.trim().split("\n").filter(|x| !x.starts_with("#")).collect();
    return configuration;
}

pub fn get_value(config: Vec<&str>) -> String {
    todo!("figure out how to parse the vec");
}
