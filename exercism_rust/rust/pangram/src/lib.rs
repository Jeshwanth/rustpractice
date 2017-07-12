pub fn is_pangram(buf: &str) -> bool {

    let refstr = "abcdefghijklmnopqrstuvwxyz";
    let mut buff: String = buf.to_string();
    let mut newstr = "".to_string();
    let chars = buff.chars();
    for c in chars {
        if c.is_alphabetic() && c <= 'z' {
            newstr.push(c);
            println!("pushed {}", c);
        } else {
            continue;
        }
    }
    let mut lower = newstr.to_lowercase();
    let mut chars: Vec<char> = lower.chars().collect();
    chars.sort();
    chars.dedup();
    lower = chars.into_iter().collect();

    println!("{}", lower);

    if refstr == lower { true } else { false }
}
