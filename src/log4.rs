extern crate time;

use std::assert;
use time::now;

pub fn log(typeOf: &str, threadType: &str, message: &str) {
    assert!(typeOf == "info" || typeOf == "warn" || typeOf == "error");
    println!("({}:{}:{}) [{} in {}] {}", now().tm_hour, now().tm_min, now().tm_sec, typeOf, threadType, message);
}