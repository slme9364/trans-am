extern crate ncurses;

mod view;
mod mode;
mod command;

use std::char;
use ncurses::*;

fn main() {
    view::init_view();
    /* Prompt for a character. */
    printw("Enter a character within 2 seconds: ");

    /* Wait for input. */
    let ch = wget_wch(stdscr());
    match ch {
        Some(WchResult::KeyCode(KEY_MOUSE)) => {
            /* Enable attributes and output message. */
            attron(A_BOLD() | A_BLINK());
            printw("\nMouse");
            attroff(A_BOLD() | A_BLINK());
            printw(" pressed");
        }

        Some(WchResult::KeyCode(KEY_UP)) => {
            /* Enable attributes and output message. */
            attron(A_BOLD() | A_BLINK());
            printw("\nKeycode");
            attroff(A_BOLD() | A_BLINK());
            printw(" UP");
        }

        Some(WchResult::KeyCode(_)) => {
            /* Enable attributes and output message. */
            attron(A_BOLD() | A_BLINK());
            printw("\nKeycode");
            attroff(A_BOLD() | A_BLINK());
            printw(" pressed");
        }

        Some(WchResult::Char(c)) => {
            /* Enable attributes and output message. */
            printw("\nKey pressed: ");
            attron(A_BOLD() | A_BLINK());
            printw(format!("{}\n", char::from_u32(c as u32).expect("Invalid char")).as_ref());
            attroff(A_BOLD() | A_BLINK());
        }

        None => {
            printw("\nYou didn't enter a character in time!");
        }
    }

    /* Refresh, showing the previous message. */
    refresh();

    /* Wait for one more character before exiting. Disable the input timeout. */
    nocbreak();
    getch();
    endwin();
}
