struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!("both Integer x: {} y: {}",both_integer.x,both_integer.y );
    println!("both float x: {} y: {}",both_float.x,both_float.y );
    println!("Intger and float x: {} y: {}",integer_and_float.x,integer_and_float.y );
}
