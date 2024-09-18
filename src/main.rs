use std::default;

fn main() {
    println!("Welcome to Mars");
}

#[derive(Default)]
struct Rover {
    position: Position,
    direction: Direction,
    boundary: i32,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Rover {
    fn with_boundaries(boundary: i32) -> Self {
        Rover {
            position: Position { x: 0, y: 0 },
            direction: Direction::NORTH,
            boundary,
        }
    }
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

impl Rover {
    fn position(&self) -> Position {
        self.position
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::NORTH => Direction::WEST,
            Direction::EAST => Direction::NORTH,
            Direction::SOUTH => Direction::EAST,
            Direction::WEST => Direction::SOUTH,
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::NORTH => {
                let current_position = self.position();
                if current_position.y == self.boundary {
                    self.position.y = -self.position.y
                } else {
                    self.position.move_north()
                }
            }
            Direction::EAST => self.position.move_east(),
            Direction::SOUTH => self.position.move_south(),
            Direction::WEST => self.position.move_west(),
        };
    }

    fn move_backward(&mut self) {
        match self.direction {
            Direction::NORTH => self.position.move_south(),
            Direction::EAST => self.position.move_west(),
            Direction::SOUTH => self.position.move_north(),
            Direction::WEST => self.position.move_east(),
        }
    }

    fn accept_commands(&mut self, commands: &[char]) {
        for i in 0..commands.len() {
            match commands[i] {
                'l' => self.turn_left(),
                'r' => self.turn_right(),
                'f' => self.move_forward(),
                'b' => self.move_backward(),
                _ => {}
            }
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
enum Direction {
    #[default]
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn instantiate_the_rover() {
        let rover = Rover::with_boundaries(10);

        assert_eq!(rover.position(), Position { x: 0, y: 0 });
        assert_eq!(rover.direction(), Direction::NORTH);
    }

    #[rstest]
    #[case(&['l'], Direction::WEST)]
    #[case(&['r'], Direction::EAST)]
    #[case(&['l', 'r'], Direction::NORTH)]
    #[case(&['l', 'l'], Direction::SOUTH)]
    #[case(&['r', 'r'], Direction::SOUTH)]
    #[case(&['r', 'r', 'r'], Direction::WEST)]
    #[case(&['r', 'r', 'r', 'r'], Direction::NORTH)]
    fn turning(#[case] commands: &[char], #[case] expected_direction: Direction) {
        let mut rover = Rover::with_boundaries(10);

        rover.accept_commands(&commands);

        assert_eq!(rover.direction(), expected_direction);
    }

    #[rstest]
    #[case(&['b'], Position { x: 0, y: -1 })]
    #[case(&['f'], Position { x: 0, y: 1})]
    #[case(&['f', 'b'], Position { x: 0, y: 0})]
    #[case(&['r', 'b'], Position {x: -1, y: 0})]
    #[case(&['l', 'b'], Position {x:1,y: 0})]
    #[case(&['r', 'r', 'b'], Position {x:0,y: 1})]
    #[case(&['r', 'f'], Position {x:1,y: 0})]
    #[case(&['r', 'r', 'f'], Position {x: 0, y:-1})]
    #[case(&['r', 'r', 'r', 'f'], Position {x:-1, y: 0})]
    fn turning_and_moving(#[case] commands: &[char], #[case] expected: Position) {
        let mut rover = Rover::with_boundaries(10);

        rover.accept_commands(&commands);

        assert_eq!(rover.position(), expected);
    }

    /// xxx x0x xxx
    /// x0x xxx xxx
    /// xxx xxx x0x
    #[test]
    fn wrap_if_reaching_the_end_of_the_planet() {
        let mut rover = Rover::with_boundaries(1);

        rover.accept_commands(&['f', 'f', 'f']);

        assert_eq!(rover.position(), Position { x: 0, y: -1 })
    }
}
