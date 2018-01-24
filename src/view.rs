extern crate ncurses;

use ncurses::*;
use cursor::Cursor;


pub fn init_view(text: &Vec<String>) {
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
    scrollok(stdscr(), true);

    let mut size = (getmaxy(stdscr()) - 1) as usize;
    if text.len() < size {
        size = text.len();
    }
    for i in 0..size {
        addstr(text[i].as_str());
        addstr("\n");
    }
    mv(0, 0);
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

pub fn optimize_relative_cursor(rcursor: &mut Cursor,
                                acursor: &Cursor,
                                text: &Vec<String>,
                                was_first: &bool,
                                y: &i32,
                                x: &i32,
                                ay: &i32) {
    if rcursor.x < 0 {
        rcursor.x = 0;
    }
    if rcursor.x > *x {
        rcursor.x = 0;
        rcursor.y += 1;
    }

    if rcursor.y < 0 {
        rcursor.y = 0;
        if acursor.y > 0 || *was_first {
            scrl(-1);
            mv(0, 0);
            addstr(text[acursor.y as usize].as_str());
        }
    }
    if rcursor.y > *y {
        if acursor.y < *ay {
            scrl(1);
            mv(*y, 0);
            addstr(text[acursor.y as usize].as_str());
            addstr("\n");
        }
        rcursor.y = *y;
    }
    if rcursor.y > *ay {
        rcursor.y = *ay;
    }
}

pub fn optimize_absolute_cursor(cursor: &mut Cursor, y: &i32, x: &i32) {
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
        cursor.y = *y;
    }
}
