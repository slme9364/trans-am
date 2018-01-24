extern crate ncurses;

use std::char;

use ncurses::*;
use KEY_CODE_TABLE;
use cursor::Cursor;
use mode::Mode;
use view;

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
            return Command {
                       ctype: CommandType::KeyCode,
                       cval: KEY_CODE_TABLE[id as usize].to_owned(),
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

pub fn insert_exec_command(_command: &mut Command, _cursor: &mut Cursor, _mode: &mut Mode) {}

pub fn normal_exec_command(_command: &mut Command,
                           _relative_cursor: &mut Cursor,
                           _absolute_cursor: &mut Cursor,
                           _mode: &mut Mode,
                           text: &Vec<String>)
                           -> bool {
    match _command.ctype {
        CommandType::Char => {
            match _command.cval.as_str() {
                "i" => *_mode = Mode::Insert,
                "h" | "BackSpace" => _command.ctype = CommandType::Left,
                "j" | "\n" => _command.ctype = CommandType::Down,
                "k" => _command.ctype = CommandType::Up,
                "l" | " " => _command.ctype = CommandType::Right,
                "q" => _command.ctype = CommandType::Exit,
                _ => (),
            }
        }
        _ => (),
    }

    let windows_size = view::get_window_size();
    let max_y = windows_size.0;
    let max_x = windows_size.1;
    let mut was_first = false;

    match _command.ctype {
        CommandType::Up => {
            _relative_cursor.y -= 1;
            if _absolute_cursor.y == 1 {
                was_first = true;
            }
            _absolute_cursor.y -= 1;
        }
        CommandType::Down => {
            _relative_cursor.y += 1;
            _absolute_cursor.y += 1;
        }
        CommandType::Left => {
            _relative_cursor.x -= 1;
            _absolute_cursor.x -= 1;
        }
        CommandType::Right => {
            _relative_cursor.x += 1;
            _absolute_cursor.x += 1;
        }
        CommandType::Exit => return false,
        _ => (),
    }
    view::optimize_absolute_cursor(_absolute_cursor, &((text.len() - 1) as i32), &max_x);
    view::optimize_relative_cursor(_relative_cursor,
                                   _absolute_cursor,
                                   &text,
                                   &was_first,
                                   &max_y,
                                   &max_x,
                                   &((text.len() - 1) as i32));
    true
}
