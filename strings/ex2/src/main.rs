fn main() {
    let buf = "10A2B3CD".chars();
    let mut digit: u32 = 0;
    println!("10A2B3CD");
    let mut result: String = "".to_string();
    let mut tmpstr: String = "".to_string();
    for c in buf
    {
        if c.is_digit(10)
        {
            tmpstr.push(c);
            digit = tmpstr.parse().unwrap();
        }
        else
        {
            if digit != 0
            {
                for _ in 0..digit
                {
                    result.push(c);
                }
            }
            else
            {
                result.push(c);
            }
            digit = 0;
            tmpstr = "".to_string();
        }
    }
    println!("{}",result);
}
