enum CommandType {
    Up,
    Down,
    Left,
    Right,
    Insert,
    Normal,
    Exit,
    Other,
}

pub struct Command {
    ctype: CommandType,
    cval: char,
}
