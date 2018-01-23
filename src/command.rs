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

pub fn normal_exec_command(_command: &mut Command, _cursor: &mut Cursor, _mode: &mut Mode) -> bool {
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

    match _command.ctype {
        CommandType::Up => _cursor.y -= 1,
        CommandType::Down => _cursor.y += 1,
        CommandType::Left => _cursor.x -= 1,
        CommandType::Right => _cursor.x += 1,
        CommandType::Exit => return false,
        _ => (),
    }
    view::optimize_cursor(_cursor, &max_y, &max_x);
    true
}
