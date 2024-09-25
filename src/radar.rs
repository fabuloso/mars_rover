#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Radar {
    boundary: i32,
    position: Position,
}

impl Radar {
    pub fn new(boundary: i32, position: Position) -> Self {
        Radar { boundary, position }
    }

    pub fn position(&self) -> Position {
        self.position
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

    pub fn move_east(&mut self) {
        self.position.move_east();
    }
    pub fn move_west(&mut self) {
        self.position.move_west();
    }
    pub fn move_north(&mut self) {
        self.position.move_north();
    }
    pub fn move_south(&mut self) {
        self.position.move_south();
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
