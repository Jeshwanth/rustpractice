use std::str;

pub fn verse(num: u32) -> String {
    if num == 0
    {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    }
    else if num == 1
    {
        return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string();
    }
    else
    {

    let mut owned_string: String = "".to_owned();
    let first_string: String = num.to_string().to_owned();
    let second_string: String = " bottles of beer on the wall, ".to_owned();
    let third_string: String = " bottles of beer.\nTake one down and pass it around, ".to_owned();
    let fourth_string: String = (num-1).to_string().to_owned();
    let mut fifth_string: String = "".to_owned();
    if (num - 1) == 1 {
        fifth_string = " bottle of beer on the wall.\n".to_string();
    }
    else
    {
        fifth_string = " bottles of beer on the wall.\n".to_string();
    }

    owned_string.push_str(&first_string);
    owned_string.push_str(&second_string);
    owned_string.push_str(&first_string);
    owned_string.push_str(&third_string);
    owned_string.push_str(&fourth_string);
    owned_string.push_str(&fifth_string);

    return owned_string;
    }
}

pub fn sing(num1: u32, num2: u32) -> String {
    let mut tempnum1 = num1;
    let mut owned_string: String = "".to_owned();
    let mut done = false;
    while !done{
        owned_string = owned_string + &verse(tempnum1);

        if tempnum1 == num2
        {
            done = true;
        }
        else
        {
            owned_string.push_str("\n");
            tempnum1 = tempnum1 - 1;
        }
    }
    return owned_string;
}
