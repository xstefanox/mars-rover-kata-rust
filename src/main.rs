fn main() {
    println!("Hello, world!");
}

struct MarsRover {
    position: Position,
}

#[derive(PartialEq, Debug)]
struct Position {
    x: u8,
    y: u8,
}

impl Default for MarsRover {
    fn default() -> Self {
        MarsRover {
            position: Position::default(),
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
    use crate::{MarsRover, Position};

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
            }
        };

        assert_eq!(mars_rover.position, Position {
            x: 1,
            y: 2,
        });
    }
}
