#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Radar {
    boundary: i32,
    pub position: Position,
}

impl Radar {
    pub fn new(boundary: i32, position: Position) -> Self {
        Radar { boundary, position }
    }

    pub fn wrap_around_edge(&mut self) {
        if self.position.y > self.boundary {
            self.position.y = -self.boundary
        }
        if self.position.y < -self.boundary {
            self.position.y = self.boundary
        }
        if self.position.x > self.boundary {
            self.position.x = -self.boundary
        }
        if self.position.x < -self.boundary {
            self.position.x = self.boundary
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn move_east(&mut self) {
        self.x = self.x + 1;
    }
    pub fn move_west(&mut self) {
        self.x = self.x - 1;
    }
    pub fn move_north(&mut self) {
        self.y = self.y + 1;
    }
    pub fn move_south(&mut self) {
        self.y = self.y - 1;
    }
}
