fn main() {
    let name = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB".to_string();
    println!("{ }",name );

    let mut chars = name.chars();
    let mut result = String::from("");
    let mut count = 0;


    let mut previous: char = '\0';
    for c in chars
    {
        if previous =='\0'
        {
            previous = c;
            println!("{ }",c);
            count = 1;
        }
        else if c == previous
        {
            count = count + 1;
            println!("match --- { } count { }",c, count);
        }
        else
        {
            println!("{} Not equal",c);

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
    println!("{ }",result );

}
