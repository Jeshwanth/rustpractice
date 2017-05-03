
extern crate curl;
use curl::easy::Easy;
use std::io::{stdout, Write};


fn main() {
    let mut handle = Easy::new();
    handle.url("https://www.rust-lang.org/").unwrap();
    handle.write_function(|data| {
    Ok(stdout().write(data).unwrap())
        }).unwrap();
    handle.perform().unwrap();
}
