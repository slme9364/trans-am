pub struct Cursor {
    pub x: i32,
    pub y: i32,
}

impl Cursor {
    pub fn new(pos_x: i32, pos_y: i32) -> Cursor {
        Cursor {
            x: pos_x,
            y: pos_y,
        }
    }
}
