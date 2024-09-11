fn main() {
    println!("Welcome to Mars");
}

#[derive(Default)]
struct Rover {
    position: Position,
    direction: Direction,
}
#[derive(Debug, Default, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
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

impl PartialEq<(i32, i32)> for Position {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self.x == other.0 && self.y == other.1
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
            Direction::NORTH => self.position.move_north(),
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
        let rover = Rover::default();

        assert_eq!(rover.position(), (0, 0));
        assert_eq!(rover.direction(), Direction::NORTH);
    }

    #[test]
    fn turn_left() {
        let mut rover = Rover::default();

        let commands = ['l'];
        rover.accept_commands(&commands);

        assert_eq!(rover.direction(), Direction::WEST);
    }

    #[test]
    fn turn_right() {
        let mut rover = Rover::default();

        let commands = ['r'];
        rover.accept_commands(&commands);

        assert_eq!(rover.direction(), Direction::EAST);
    }

    #[test]
    fn turn_left_and_turn_right() {
        let mut rover = Rover::default();

        let commands = ['l', 'r'];
        rover.accept_commands(&commands);

        assert_eq!(rover.direction(), Direction::NORTH);
    }

    #[test]
    fn turn_left_four_times() {
        let mut rover = Rover::default();

        let commands = ['l', 'l', 'l', 'l'];
        rover.accept_commands(&commands);

        assert_eq!(rover.direction(), Direction::NORTH);
    }

    #[test]
    fn when_command_is_forward_and_rover_facing_north_the_rover_moves_forward() {
        let mut rover = Rover::default();

        let commands = ['f'];
        rover.accept_commands(&commands);

        assert_eq!(rover.position(), (0, 1));
    }

    #[test]
    fn when_command_is_forward_and_rover_facing_east_the_rover_moves_forward() {
        let mut rover = Rover::default();

        let commands = ['r', 'f'];
        rover.accept_commands(&commands);

        assert_eq!(rover.position(), (1, 0));
    }

    #[test]
    fn when_command_is_forward_and_rover_facing_south_the_rover_moves_forward() {
        let mut rover = Rover::default();

        let commands = ['r', 'r', 'f'];
        rover.accept_commands(&commands);

        assert_eq!(rover.position(), (0, -1));
    }

    #[test]
    fn when_command_is_forward_and_rover_facing_west_the_rover_moves_forward() {
        let mut rover = Rover::default();

        let commands = ['l', 'f'];
        rover.accept_commands(&commands);

        assert_eq!(rover.position(), (-1, 0));
    }

    #[rstest]
    #[case(&['b'], (0, -1))]
    #[case(&['f', 'b'], (0, 0))]
    #[case(&['r', 'b'], (-1, 0))]
    #[case(&['l', 'b'], (1, 0))]
    #[case(&['r', 'r', 'b'], (0, 1))]
    fn when_command_is_backward_and_rover_facing_north_the_rover_moves_backward(
        #[case] commands: &[char],
        #[case] expected: (i32, i32),
    ) {
        let mut rover = Rover::default();

        rover.accept_commands(&commands);

        assert_eq!(rover.position(), expected);
    }
}
