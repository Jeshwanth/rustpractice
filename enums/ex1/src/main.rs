enum Message {
   Quit,
   ChangeColor(i32, i32, i32),
   Move { x: i32, y: i32 },
   Write(String),
}

fn main() {

let x: Message = Message::Quit;
let y: Message = Message::ChangeColor(1,2,3);
let z: Message = Message::Move{x: 3,y: 4};
let p: Message = Message::Write("Rustacian".to_string());

enumtest(x);
enumtest(y);
enumtest(z);
enumtest(p);

}

fn enumtest(x: Message){
    match x {
        Message::Quit => println!("Quit"),
        Message::ChangeColor(r,g,b) => println!("ChangeColor r = {}, g = {}, b = {}", r,g,b),
        Message::Move{x,y} => println!("x = {}, y = {}",x,y),
        Message::Write(s) => println!("The passed string is {}",s),
    };

}
