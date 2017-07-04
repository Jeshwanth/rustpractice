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

    let mut owned_string: String = num.to_string().to_owned();
    let another_owned_string: String = " bottles of beer on the wall,".to_owned();
    let onemore: String = "bottles of beer.\nTake one down and pass it around,".to_owned();

    owned_string.push_str(&another_owned_string);
    owned_string.push_str(&onemore);
    return owned_string;
    }
}

pub fn sing(num1: u32, num2: u32) -> String {
    return "Hello".to_string();
}
