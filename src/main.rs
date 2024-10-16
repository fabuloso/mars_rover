mod radar;
use radar::*;


fn main() {
    let mut rover = Rover::with_boundaries(10);
    rover.accept_commands(&['l', 'r']);
    rover.position();
    rover.direction();
}


enum RoverError {
    ObstacleHit(ObstacleHit)
}

#[derive(Default)]
struct Rover {
    radar: Radar,
}

impl Rover {
    fn with_boundaries(boundary: i32) -> Self {
        Rover {
            radar: Radar::new(boundary, Position { x: 0, y: 0 }, Direction::NORTH),
        }
    }
    fn with_radar(radar: Radar) -> Self {
        Rover { radar }
    }
}

impl Rover {
    pub fn accept_commands(&mut self, commands: &[char]) -> Result<(), RoverError> {
        for i in 0..commands.len() {
            match commands[i] {
                'l' => self.turn_left(),
                'r' => self.turn_right(),
                'f' => {
                    self.move_forward()?;
                },
                'b' => {
                    self.move_backward()?;
                },
                _ => ()
            }
        }
        Ok(())
    }

    fn position(&self) -> Position {
        self.radar.position()
    }

    fn direction(&self) -> Direction {
        self.radar.direction()
    }

    fn turn_left(&mut self) {
        self.radar.turn_left();
    }

    fn turn_right(&mut self) {
        self.radar.turn_right();
    }

    fn move_forward(&mut self) -> Result<(), RoverError> {
        self.radar.move_forward().map_err(RoverError::ObstacleHit)
    }

    fn move_backward(&mut self) -> Result<(), RoverError> {
        self.radar.move_backward().map_err(RoverError::ObstacleHit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use radar::*;
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

    #[rstest]
    #[case(Direction::NORTH, Position {x:0, y:1})]
    #[case(Direction::EAST, Position {x:1, y:0})]
    #[case(Direction::WEST, Position {x:-1, y:0})]
    #[case(Direction::SOUTH, Position {x:0, y:-1})]
    fn stops_when_reaching_an_obstacle(#[case] starting_direction: Direction, #[case] obstacle: Position) {
        let radar = Radar::new_with_obstacles(
            1,
            Position { x: 0, y: 0 },
            starting_direction,
            vec![obstacle],
        );
        let mut rover = Rover::with_radar(radar);

        let result = rover.accept_commands(&['f']);

        assert_eq!(rover.position(), Position { x: 0, y: 0 })
    }

    #[rstest]
    #[case(Direction::NORTH, &['f', 'l', 'f'], vec![Position {x:0,y:1}])]
    #[case(Direction::NORTH, &['f', 'b', 'b','l','b'], vec![Position {x:0,y:-1}])]

    fn when_hitting_an_obstacle_should_abort_the_sequence(#[case] starting_direction: Direction, #[case] commands: &[char], #[case] obstacles: Vec<Position>) {
        let radar = Radar::new_with_obstacles(
            1,
            Position { x: 0, y: 0 },
            starting_direction,
            obstacles,
        );
        let mut rover = Rover::with_radar(radar);

        let result = rover.accept_commands(commands);

        assert!(matches!(result, Err(RoverError::ObstacleHit(_))));
        assert_eq!(rover.position(), Position { x: 0, y: 0 });
        assert_eq!(rover.direction(), Direction::NORTH);
    }

    
}
