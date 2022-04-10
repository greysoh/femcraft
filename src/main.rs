use std::env;
use std::fs;
mod parse_config;
mod log4;

fn main() {
    log4::log("info", "main", "Hello, world!");
}
