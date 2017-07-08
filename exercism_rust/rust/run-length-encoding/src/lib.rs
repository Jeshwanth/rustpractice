pub fn encode(buf: &str) -> String
{

    let mut chars = buf.chars();
    let mut result = String::from("");
    let mut count = 0;

    if buf == ""
    {
        return "".to_string();
    }
    let mut previous: char = '\0';
    for c in chars
    {
        if previous =='\0'
        {
            previous = c;
            count = 1;
        }
        else if c == previous
        {
            count = count + 1;
        }
        else
        {
            if count > 0
            {
                if count > 1 { result += &count.to_string();}
                result += &previous.to_string();
                count = 0;
            }
            previous = c;
            count = count + 1;
        }

    }
    if count > 1 {result += &count.to_string();}
    result += &previous.to_string();
    return result.to_string();
}


pub fn decode(buf: &str) -> String
{
    let mut digit: u32 = 0;
    let mut chars = buf.chars();
    let mut result: String = "".to_string();
    let mut tmpstr: String = "".to_string();

    if buf == ""
    {
        return "".to_string();
    }
    for c in chars
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
    return result.to_string();
}
