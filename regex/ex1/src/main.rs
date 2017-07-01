extern crate regex;

use regex::Regex;

fn main() {
    println!("Hello, Regex!");

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}\?$").unwrap();
    if re.is_match("2014-01-01?")
    {
        println!("Thats a match");
    }
    else
    {
        println!("NoMatch");
    }

}
