pub struct Square {
    x: u32
}

//
// implementation for square
//
impl Square {
    pub fn get_square(&self) -> u32 {
        self.x * self.x
    }
}


fn main() {
    println!("Hello, impl!");

    let number = 15;

    let mystruct = Square{x: number};

    let result = mystruct.get_square();

    println!("The square of {} is {}",number,result );

}
