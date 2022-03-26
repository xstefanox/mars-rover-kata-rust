fn main() {
    println!("Hello, world!");
}

struct MarsRover {
    x: u8,
    y: u8,
}

impl Default for MarsRover {
    fn default() -> Self {
        MarsRover {
            x: 0,
            y: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MarsRover;

    #[test]
    fn default_initial_position() {
        let mars_rover = MarsRover::default();

        assert_eq!(mars_rover.x, 0);
        assert_eq!(mars_rover.y, 0);
    }
}
