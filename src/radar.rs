type RadarResult = Result<(), ObstacleHit>;


#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub enum Direction {
    #[default]
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

pub struct ObstacleHit;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Radar {
    boundary: i32,
    position: Position,
    direction: Direction,
    obstacles: Vec<Position>,
}

impl Radar {
    pub fn new(boundary: i32, position: Position, direction: Direction) -> Self {
        Radar {
            boundary,
            position,
            direction,
            obstacles: Vec::new(),
        }
    }

    pub fn new_with_obstacles(
        boundary: i32,
        position: Position,
        direction: Direction,
        obstacles: Vec<Position>,
    ) -> Self {
        Radar {
            boundary,
            position,
            direction,
            obstacles,
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

    pub fn move_forward(&mut self) -> RadarResult {
        let next_position = match self.direction {
            Direction::NORTH => self.position.move_north(),
            Direction::EAST => self.position.move_east(),
            Direction::SOUTH => self.position.move_south(),
            Direction::WEST => self.position.move_west(),
        };

        self.try_move(next_position)
    }

    pub fn move_backward(&mut self) -> RadarResult {
        let next_position = match self.direction {
            Direction::NORTH => self.position.move_south(),
            Direction::EAST => self.position.move_west(),
            Direction::SOUTH => self.position.move_north(),
            Direction::WEST => self.position.move_east(),
        };

        self.try_move(next_position)
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

    fn try_move(&mut self, next_position: Position) -> RadarResult {
        if self.obstacles.contains(&next_position) {
            return Err(ObstacleHit)
        }

        self.position = next_position;
        self.wrap_around_edge();
        Ok(())
    } 
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn move_east(&self) -> Position {
        Self { x: self.x + 1, ..*self }
    }
    pub fn move_west(&self) -> Position {
        Self { x: self.x - 1, ..*self }
    }
    pub fn move_north(&self) -> Position {
        Self { y: self.y + 1, ..*self }
    }
    pub fn move_south(&self) -> Position {
        Self { y: self.y - 1, ..*self }
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
