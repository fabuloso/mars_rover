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

    fn accept_command(&mut self, commands: &[char]) {
        self.direction = Direction::WEST;
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
        rover.accept_command(&commands);

        assert_eq!(rover.direction(), Direction::WEST);
    }
}
