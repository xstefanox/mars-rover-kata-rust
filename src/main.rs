fn main() {
    println!("Hello, world!");
}

struct MarsRover {
    position: Position,
    direction: Direction,
}

#[derive(PartialEq, Debug)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Default for MarsRover {
    fn default() -> Self {
        MarsRover {
            position: Position::default(),
            direction: Direction::N,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position {
            x: 0,
            y: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::{Direction, MarsRover, Position};

    #[test]
    fn default_initial_position() {
        let mars_rover = MarsRover::default();

        assert_eq!(mars_rover.position, Position {
            x: 0,
            y: 0,
        });
    }

    #[test]
    fn given_initial_position() {
        let mars_rover = MarsRover {
            position: Position {
                x: 1,
                y: 2,
            },
            ..MarsRover::default()
        };

        assert_eq!(mars_rover.position, Position {
            x: 1,
            y: 2,
        });
    }

    #[test]
    fn default_initial_direction() {
        let mars_rover = MarsRover::default();

        assert_eq!(mars_rover.direction, Direction::N)
    }

    #[rstest]
    #[case(Direction::N)]
    #[case(Direction::S)]
    #[case(Direction::W)]
    #[case(Direction::E)]
    fn given_initial_direction(#[case] direction: Direction) {
        let mars_rover = MarsRover {
            direction,
            ..MarsRover::default()
        };

        assert_eq!(mars_rover.direction, direction)
    }
}
