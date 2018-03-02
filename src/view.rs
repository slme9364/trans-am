extern crate ncurses;

use ncurses::*;
use cursor::Cursor;
use mode::Mode;
use file;
use {MAIN_WINDOW, INFO_WINDOW, LINE_NO_WINDOW};

pub struct View {
    pub acursor: Cursor,
    pub rcursor: Cursor,
    pub mode: Mode,
    pub windows: [WINDOW; 3],
}

impl View {
    pub fn new() -> View {
        View {
            acursor: Cursor { x: 0, y: 0 },
            rcursor: Cursor { x: 0, y: 0 },
            mode: Mode::Normal,
            windows: create_window(),
        }
    }
    pub fn status(&self) {
        let mode_str = match self.mode {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
        };
        wmove(self.windows[INFO_WINDOW], 0, 0);
        waddstr(self.windows[INFO_WINDOW],
                &format!("{} : {}", mode_str, file::get_file_name()));
        wrefresh(self.windows[INFO_WINDOW]);
    }
}

fn create_window() -> [WINDOW; 3] {
    let line_no = file::get_file_line_no();
    let length = (format!("{}", line_no).len() + 1) as i32;
    let divide_line = 1;
    let lineno_win = newwin(getmaxy(stdscr()) - divide_line, length, 0, 0);
    let info_win = newwin(divide_line,
                          getmaxx(stdscr()),
                          getmaxy(stdscr()) - divide_line,
                          0);
    let main_win = newwin(getmaxy(stdscr()) - divide_line,
                          getmaxx(stdscr()) - length,
                          0,
                          length);
    [lineno_win, info_win, main_win]
}

pub fn init_view(text: &Vec<String>, windows: &[WINDOW; 3]) {
    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "en_US.UTF-8");

    /* Setup ncurses. */
    raw();

    /* Require input within 2 seconds. */
    halfdelay(20);
    /* Enable mouse events. */
    mousemask(ALL_MOUSE_EVENTS as mmask_t, None);

    /* Allow for extended keyboard (like F1). */
    keypad(windows[MAIN_WINDOW], true);
    noecho();

    // Scroll available
    scrollok(windows[MAIN_WINDOW], true);
    scrollok(windows[LINE_NO_WINDOW], true);

    // Initialize View
    waddstr(windows[INFO_WINDOW],
            &format!("NORMAL : {}\n", file::get_file_name()));
    wrefresh(windows[INFO_WINDOW]);
    let mut size = (getmaxy(windows[MAIN_WINDOW]) - 1) as usize;
    if text.len() < size {
        size = text.len();
    }

    let length = (format!("{}", file::get_file_line_no()).len() + 1) as i32;
    for i in 0..size {
        waddstr(windows[LINE_NO_WINDOW],
                &format!("{number:>length$} ",
                         number = i + 1,
                         length = (length - 1) as usize));
        wrefresh(windows[LINE_NO_WINDOW]);
        waddstr(windows[MAIN_WINDOW], text[i].as_str());
        waddstr(windows[MAIN_WINDOW], "\n");
        wrefresh(windows[MAIN_WINDOW]);
    }
    wmove(windows[MAIN_WINDOW], 0, 0);
    wrefresh(windows[MAIN_WINDOW]);
}

pub fn get_key(win: WINDOW) -> Option<WchResult> {
    wget_wch(win)
}

pub fn get_window_size(window: WINDOW) -> (i32, i32) {
    let mut y = 0;
    let mut x = 0;
    getmaxyx(window, &mut y, &mut x);
    (y, x)
}

pub fn optimize_relative_cursor(rcursor: &mut Cursor,
                                acursor: &Cursor,
                                text: &Vec<String>,
                                mwindow: &WINDOW,
                                lwindow: &WINDOW,
                                was_first: &bool) {
    let x = text[acursor.y as usize].len() as i32; // getmaxx(stdscr());
    let y = getmaxy(*mwindow) - 1;
    let line_size = getmaxx(*lwindow);
    let ay = (text.len() - 1) as i32;

    if rcursor.x < 0 {
        rcursor.x = 0;
    }
    if rcursor.x > x {
        rcursor.x = 0;
    }

    if rcursor.y < 0 {
        rcursor.y = 0;
        if acursor.y > 0 || *was_first {
            wscrl(*mwindow, -1);
            wmove(*mwindow, 0, 0);
            waddstr(*mwindow, text[acursor.y as usize].as_str());
            wrefresh(*mwindow);
            wscrl(*lwindow, -1);
            wmove(*lwindow, 0, 0);
            waddstr(*lwindow,
                    &format!("{number:line_size$} ",
                             number = acursor.y + 1,
                             line_size = (line_size - 1) as usize));
            wrefresh(*lwindow);
        }
    }
    if rcursor.y > y {
        if acursor.y < ay {
            wmove(*mwindow, y, 0);
            waddstr(*mwindow, text[acursor.y as usize].as_str());
            waddstr(*mwindow, "\n");
            wrefresh(*mwindow);
            wmove(*lwindow, y, 0);
            waddstr(*lwindow,
                    &format!("{number:line_size$} ",
                             number = acursor.y + 1,
                             line_size = (line_size - 1) as usize));
            wrefresh(*lwindow);

        }
        rcursor.y = y;
    }
    if rcursor.y > ay {
        rcursor.y = ay;
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
