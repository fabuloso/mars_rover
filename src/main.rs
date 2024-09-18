use std::default;
mod radar;
use radar::*;

fn main() {
    println!("Welcome to Mars");
}

#[derive(Default)]
struct Rover {
    direction: Direction,
    radar: Radar,
}

impl Rover {
    fn with_boundaries(boundary: i32) -> Self {
        Rover {
            direction: Direction::NORTH,
            radar: Radar::new(boundary, Position { x: 0, y: 0 }),
        }
    }
}

impl Rover {
    fn position(&self) -> Position {
        self.radar.position
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
            Direction::NORTH => self.position.move_north(),
            Direction::EAST => self.position.move_east(),
            Direction::SOUTH => self.position.move_south(),
            Direction::WEST => self.position.move_west(),
        };
        self.wrap_around_edge();
    }

    fn move_backward(&mut self) {
        match self.direction {
            Direction::NORTH => self.position.move_south(),
            Direction::EAST => self.position.move_west(),
            Direction::SOUTH => self.position.move_north(),
            Direction::WEST => self.position.move_east(),
        }
        self.wrap_around_edge();
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

    fn wrap_around_edge(&mut self) {
        if self.position().y > self.boundary {
            self.position.y = -self.boundary
        }
        if self.position().y < -self.boundary {
            self.position.y = self.boundary
        }
        if self.position().x > self.boundary {
            self.position.x = -self.boundary
        }
        if self.position().x < -self.boundary {
            self.position.x = self.boundary
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
    /// xxx xxx xx0
    #[rstest]
    #[case(&['f','f'], Position {x:0, y:-1})]
    #[case(&['r', 'f','f'], Position {x:-1, y:0})]
    #[case(&['l', 'f','f'], Position {x:1, y:0})]
    #[case(&['l', 'l', 'f','f'], Position {x:0, y:1})]
    #[case(&['r', 'f', 'r','f', 'f'], Position {x:1, y:1})]
    #[case(&['r', 'f', 'r','f', 'f', 'l', 'f'], Position {x:-1, y:1})]
    #[case(&['b','b'], Position {x:0, y:1})]
    #[case(&['r', 'b','b'], Position {x:1, y:0})]
    #[case(&['l', 'b','b'], Position {x:-1, y:0})]
    #[case(&['l', 'l', 'b','b'], Position {x:0, y:-1})]
    fn wrap_if_reaching_the_end_of_the_planet(
        #[case] commands: &[char],
        #[case] expected: Position,
    ) {
        let mut rover = Rover::with_boundaries(1);

        rover.accept_commands(commands);

        assert_eq!(rover.position(), expected)
    }
    /// xxx x0x xxx xxx
    /// x0x xxx xxx x0x
    /// xxx xxx x0x xxx
    #[test]
    fn wrap_if_reaching_the_end_of_the_planet_til_the_center_of_the_map() {
        let mut rover = Rover::with_boundaries(1);

        rover.accept_commands(&['f', 'f', 'f']);

        assert_eq!(rover.position(), Position { x: 0, y: 0 })
    }
}
