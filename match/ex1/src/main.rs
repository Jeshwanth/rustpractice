fn main() {
    println!("Let's learn match!");

    let number = 333;
    match number {
        1   =>  println!("One it is"),
        33  =>  println!("yes 33 it is"),
        33 | 23 => println!("Either 33 or 23"),
        1 | 3 | 5 | 7 => println!("Looks to be prime number?"),
        10...20 => println!("In the range of 10 to 20"),
        _ => unreachable!()
    }

}
