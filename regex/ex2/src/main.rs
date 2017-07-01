fn main() {
    let test_str = "Whoa, chill out!"; // type &str
    let test_string = test_str.to_string(); // type String

    let uppercase_test_string = test_string.to_uppercase(); // type String

    let uppercase_test_str = &*uppercase_test_string; // back to type &str

    println!{"{}", test_str};
    println!{"{}", uppercase_test_string};
    println!{"{}", uppercase_test_str};
}
