extern crate ncurses;

use ncurses::*;
use std::char;


fn main() {

	initscr();
	raw();

	keypad(stdscr(), true);
	noecho();

    printw("Enter a character: ");

	let ch = getch();
	if ch == KEY_F2
	{
			attron(A_BOLD() | A_BLINK());
			printw("\nF2");
			attroff(A_BOLD() | A_BLINK());
			printw("\nPressed");
	}
	else
	{
			printw("\nKey pressed");
			attron(A_BOLD() | A_BLINK());
			printw(format!("{}\n", char::from_u32(ch as u32).expect("Invalid char")).as_ref());
			attroff(A_BOLD() | A_BLINK());

	}

	refresh();

	getch();

	endwin();

}
