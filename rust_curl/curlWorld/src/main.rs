extern crate curl; //Curl crate
use curl::easy::Easy; //Using easy functions
use std::io::{stdout, Write};


fn main() {
    let mut handle = Easy::new();
    handle.url("https://www.rust-lang.org/").unwrap();
    handle.write_function(|data| {
    Ok(stdout().write(data).unwrap())
        }).unwrap();
    handle.perform().unwrap();
}
