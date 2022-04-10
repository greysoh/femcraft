extern crate time;

use std::assert;
use time::now;

pub fn log(type_of: &str, thread_type: &str, message: &str) {
    assert!(type_of == "info" || type_of == "warn" || type_of == "error");
    println!("({}:{}:{}) [{} in {}] {}", now().tm_hour, now().tm_min, now().tm_sec, type_of, thread_type, message);
}