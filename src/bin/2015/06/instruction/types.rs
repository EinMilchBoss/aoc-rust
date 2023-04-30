#[derive(Debug)]
pub struct Instruction {
    pub command: Command,
    pub coordinate_pair: CoordinatePair,
}

#[derive(Debug)]
pub enum Command {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
pub struct CoordinatePair(pub Coordinate, pub Coordinate);

#[derive(Debug)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match value {
            "turn on" => Self::On,
            "turn off" => Self::Off,
            "toggle" => Self::Toggle,
            _ => panic!("String `{}` could not be parsed to a command.", value),
        }
    }
}

impl CoordinatePair {
    pub fn area(&self) -> Vec<Coordinate> {
        let mut coordinates = Vec::new();
        let CoordinatePair(from_coordinate, to_coordinate) = self;
        for y in from_coordinate.y..=to_coordinate.y {
            for x in from_coordinate.x..=to_coordinate.x {
                coordinates.push(Coordinate { x, y });
            }
        }
        coordinates
    }
}
