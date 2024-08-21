fn main() {
    println!("Welcome to Mars");
}

#[derive(Default)]
struct Rover {
    position: (u32, u32),
    direction: Direction,
}

impl Rover {
    fn position(&self) -> (u32, u32) {
        self.position
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn move_left(&mut self) {
        self.direction = match self.direction {
            Direction::NORTH => Direction::WEST,
            Direction::EAST => Direction::NORTH,
            Direction::SOUTH => Direction::EAST,
            Direction::WEST => Direction::SOUTH,
        }
    }

    fn move_right(&mut self) {
        self.direction = match self.direction {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }

    fn accept_commands(&mut self, commands: &[char]) {
        for i in 0..commands.len() {
            match commands[i] {
                'l' => self.move_left(),
                'r' => self.move_right(),
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
}
