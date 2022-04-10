extern crate chrono;
use std::assert;

pub fn log(type_of: &str, thread_type: &str, message: &str) {
    let time = chrono::DateTime::time(&chrono::offset::Local::now());
    assert!(type_of == "info" || type_of == "warn" || type_of == "error");

    println!("({}) [{} in {}] {}", time, type_of, thread_type, message);
}
