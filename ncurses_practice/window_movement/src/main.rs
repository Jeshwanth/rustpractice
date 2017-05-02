extern crate ncurses;

use ncurses::*;

static WINDOW_HEIGHT: i32 = 3;
static WINDOW_WIDTH: i32 = 10;

fn main() {

	initscr();
	raw();

	keypad(stdscr(), true);
	noecho();

	cur
    println!("Hello, world!");
}
