extern crate regex;
use std::str;

use regex::Regex;

pub fn reply(buf: &str) -> &str
{
    if buf == ""
    {
        return "Fine. Be that way!";
    }
    else if is_question(buf)
    {
        return "Sure.";
    }
    else if is_shout(buf)
    {
        return "Whoa, chill out!";
    }
    else
    {
        return "Whatever.";
    }

}

fn is_question(buf: &str) -> bool
{
    let re = Regex::new(r".*\?$").unwrap();
    if re.is_match(buf)
    {
        return true;
    }
    else
    {
        return false;
    }
}


fn is_shout(buf: &str) -> bool
{
    let base = buf.to_string();
    let upper = base.to_uppercase();

    if base == upper
    {
        return true;
    }
    else{
        return false;
    }
}
