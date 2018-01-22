extern crate ncurses;

use ncurses::*;


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
