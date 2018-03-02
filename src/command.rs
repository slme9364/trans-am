extern crate ncurses;

use std::char;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use ncurses::*;
use KEY_CODE_TABLE;
use cursor::Cursor;
use mode::Mode;
use view;
use view::View;
use file;
use {MAIN_WINDOW, LINE_NO_WINDOW};

pub enum CommandType {
    Up,
    Down,
    Left,
    Right,
    Exit,
    KeyCode,
    Char,
    Other,
}

pub struct Command {
    pub ctype: CommandType,
    pub cval: String,
}

trait CreateCommand {
    fn create_command(&self) -> Command;
}

impl CreateCommand for String {
    fn create_command(&self) -> Command {
        Command {
            ctype: CommandType::Char,
            cval: self.clone(),
        }
    }
}

impl CreateCommand for i32 {
    fn create_command(&self) -> Command {
        let id = *self - 255;
        if id < 110 {
            let key = KEY_CODE_TABLE[id as usize];
            let types = match key {
                "Up" => CommandType::Up,
                "Down" => CommandType::Down,
                "Left" => CommandType::Left,
                "Right" => CommandType::Right,
                _ => CommandType::KeyCode,
            };
            return Command {
                       ctype: types,
                       cval: key.to_owned(),
                   };
        }
        Command {
            ctype: CommandType::Other,
            cval: "Invalid".to_owned(),
        }
    }
}

pub fn key_parse(key: Option<WchResult>) -> Command {
    match key {
        Some(WchResult::Char(c)) => {
            format!("{}", char::from_u32(c as u32).expect("Invalid char")).create_command()
        }
        Some(WchResult::KeyCode(val)) => val.create_command(),
        None => "".to_owned().create_command(),
    }
}

fn save(text: &Vec<String>, filename: &str) {
    let path = Path::new(filename);
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(_) => return,
    };
    let mut sum_text = "".to_owned();
    for data in text {
        sum_text = sum_text.clone() + data.as_str();
        sum_text = sum_text.clone() + "\n";
    }
    file.write_all(sum_text.as_bytes());
}

fn insert_str(text: &mut Vec<String>,
              word: String,
              rcursor: &mut Cursor,
              acursor: &mut Cursor,
              windows: &[WINDOW; 3]) {
    let key = word.as_str();
    let x = getmaxx(stdscr());

    if key == "\n" {
        text.insert((acursor.y + 1) as usize, "".to_owned());
        acursor.y += 1;
        rcursor.y += 1;
        acursor.x = 0;
        rcursor.x = 0;
        view::optimize_absolute_cursor(acursor, &((text.len() - 1) as i32), &x);
        view::optimize_relative_cursor(rcursor,
                                       acursor,
                                       &text,
                                       &windows[MAIN_WINDOW],
                                       &windows[LINE_NO_WINDOW],
                                       &false);
        wmove(windows[MAIN_WINDOW], rcursor.y, rcursor.x);
        winsertln(windows[MAIN_WINDOW]);

    } else if key != "" && key.len() == 1 {
        if text[acursor.y as usize].len() == 0 ||
           text[acursor.y as usize].len() <= acursor.x as usize {
            text[acursor.y as usize] = text[acursor.y as usize].clone() + key;
            acursor.x = (text[acursor.y as usize].len() - 1) as i32;
        } else {
            text[acursor.y as usize].insert_str(acursor.x as usize, key);
        }
        wmove(windows[MAIN_WINDOW], rcursor.y, 0);
        waddstr(windows[MAIN_WINDOW], text[acursor.y as usize].as_str());
        rcursor.x += 1;
        acursor.x += 1;
    }
}

fn delete_str(text: &mut Vec<String>,
              rcursor: &mut Cursor,
              acursor: &mut Cursor,
              windows: &[WINDOW; 3]) {
    let x = getmaxx(stdscr());
    if acursor.x > 0 {
        text[acursor.y as usize].remove((acursor.x - 1) as usize);
        rcursor.x -= 1;
        acursor.x -= 1;
        view::optimize_absolute_cursor(acursor, &((text.len() - 1) as i32), &x);
        view::optimize_relative_cursor(rcursor,
                                       acursor,
                                       &text,
                                       &windows[MAIN_WINDOW],
                                       &windows[LINE_NO_WINDOW],
                                       &false);
        wmove(windows[MAIN_WINDOW], rcursor.y, rcursor.x);
        wdelch(windows[MAIN_WINDOW]);
    }
}

pub fn insert_exec_command(_command: &mut Command, view: &mut View, text: &mut Vec<String>) {
    match _command.ctype {
        CommandType::Char => {
            // x1b means ESC key
            if _command.cval.as_str() == "\x1b" {
                view.mode = Mode::Normal;
            } else {
                insert_str(text,
                           _command.cval.clone(),
                           &mut view.rcursor,
                           &mut view.acursor,
                           &view.windows);
            }
        }
        CommandType::Up | CommandType::Down | CommandType::Left | CommandType::Right => {
            mv_cursor_scrl(_command,
                           &mut view.rcursor,
                           &mut view.acursor,
                           text,
                           &view.windows);
        }
        CommandType::KeyCode => {
            if _command.cval.as_str() == "BackSpace" {
                delete_str(text, &mut view.rcursor, &mut view.acursor, &view.windows);
            }
        }
        _ => (),
    }
}

pub fn normal_exec_command(_command: &mut Command, view: &mut View, text: &Vec<String>) -> bool {
    match _command.ctype {
        CommandType::Char => {
            match _command.cval.as_str() {
                "i" => view.mode = Mode::Insert,
                "h" => _command.ctype = CommandType::Left,
                "j" | "\n" => _command.ctype = CommandType::Down,
                "k" => _command.ctype = CommandType::Up,
                "l" | " " => _command.ctype = CommandType::Right,
                "q" => _command.ctype = CommandType::Exit,
                "s" => save(text, file::get_file_name().as_str()),
                _ => (),
            }
        }
        CommandType::KeyCode => {
            if _command.cval.as_str() == "BackSpace" {
                _command.ctype = CommandType::Left;
            }
        }
        _ => (),
    }

    match _command.ctype {
        CommandType::Up | CommandType::Down | CommandType::Left | CommandType::Right => {
            mv_cursor_scrl(_command,
                           &mut view.rcursor,
                           &mut view.acursor,
                           text,
                           &view.windows);
        }
        CommandType::Exit => return false,
        _ => (),
    }
    true
}

fn mv_cursor_scrl(command: &Command,
                  rcursor: &mut Cursor,
                  acursor: &mut Cursor,
                  text: &Vec<String>,
                  windows: &[WINDOW; 3]) {
    let windows_size = view::get_window_size(windows[MAIN_WINDOW]);
    let max_x = windows_size.1;
    let mut was_first = false;

    match command.ctype {
        CommandType::Up => {
            rcursor.y -= 1;
            if acursor.y == 1 {
                was_first = true;
            }
            acursor.y -= 1;
        }
        CommandType::Down => {
            rcursor.y += 1;
            acursor.y += 1;
        }
        CommandType::Left => {
            rcursor.x -= 1;
            acursor.x -= 1;
        }
        CommandType::Right => {
            rcursor.x += 1;
            acursor.x += 1;
        }
        _ => (),
    }
    view::optimize_absolute_cursor(acursor, &((text.len() - 1) as i32), &max_x);
    view::optimize_relative_cursor(rcursor,
                                   acursor,
                                   &text,
                                   &windows[MAIN_WINDOW],
                                   &windows[LINE_NO_WINDOW],
                                   &was_first);

}
