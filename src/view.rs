extern crate ncurses;

use ncurses::*;
use cursor::Cursor;


pub fn init_view() {
    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "en_US.UTF-8");

    /* Setup ncurses. */
    initscr();
    raw();

    /* Require input within 2 seconds. */
    halfdelay(20);
    /* Enable mouse events. */
    mousemask(ALL_MOUSE_EVENTS as mmask_t, None);

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

}

pub fn get_key() -> Option<WchResult> {
    wget_wch(stdscr())
}

pub fn get_window_size() -> (i32, i32) {
    let mut y = 0;
    let mut x = 0;
    getmaxyx(stdscr(), &mut y, &mut x);
    (y, x)
}

pub fn optimize_cursor(cursor: &mut Cursor, y: &i32, x: &i32) {
    if cursor.x < 0 {
        cursor.x = 0;
    }
    if cursor.x > *x {
        cursor.x = 0;
        cursor.y += 1;
    }
    if cursor.y < 0 {
        cursor.y = 0;
    }
    if cursor.y > *y {
        cursor.y = *y
    }
}
