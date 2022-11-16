use std::{fs::write, process};

pub fn write_sources() {
    let source = include_bytes!(concat!(env!("OUT_DIR"), "/source.zip"));
    write("source.zip", source).unwrap();
    process::exit(0)
}
