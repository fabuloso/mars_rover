#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub enum Direction {
    #[default]
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Radar {
    boundary: i32,
    position: Position,
    direction: Direction,
}

impl Radar {
    pub fn new(boundary: i32, position: Position, direction: Direction) -> Self {
        Radar {
            boundary,
            position,
            direction,
        }
    }

    pub fn direction(&self) -> Direction {
        self.direction
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

    pub fn move_forward(&mut self) {
        match self.direction {
            Direction::NORTH => self.move_north(),
            Direction::EAST => self.move_east(),
            Direction::SOUTH => self.move_south(),
            Direction::WEST => self.move_west(),
        };
        self.wrap_around_edge();
    }

    pub fn move_backward(&mut self) {
        match self.direction {
            Direction::NORTH => self.move_south(),
            Direction::EAST => self.move_west(),
            Direction::SOUTH => self.move_north(),
            Direction::WEST => self.move_east(),
        }
        self.wrap_around_edge();
    }

    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::NORTH => Direction::WEST,
            Direction::EAST => Direction::NORTH,
            Direction::SOUTH => Direction::EAST,
            Direction::WEST => Direction::SOUTH,
        }
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }

    fn move_east(&mut self) {
        self.position.move_east();
    }
    fn move_west(&mut self) {
        self.position.move_west();
    }
    fn move_north(&mut self) {
        self.position.move_north();
    }
    fn move_south(&mut self) {
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::*;

//     #[rstest]
//     #[case(&['l'], Direction::WEST)]
//     fn test_moving_east_changes_positions() {
//         let mut radar = Radar::new(1, Position { x: 0, y: 0 });

//         radar.move_east();

//         assert_eq!(radar.position, Position { x: -1, y: 0 })
//     }
// }
