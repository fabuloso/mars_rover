#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Radar {
    boundary: i32,
    pub position: Position,
}

impl Radar {
    pub fn new(boundary: i32, position: Position) -> Self {
        Radar { boundary, position }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn move_east(&mut self) {
        self.x = self.x + 1;
    }
    fn move_west(&mut self) {
        self.x = self.x - 1;
    }
    fn move_north(&mut self) {
        self.y = self.y + 1;
    }
    fn move_south(&mut self) {
        self.y = self.y - 1;
    }
}
