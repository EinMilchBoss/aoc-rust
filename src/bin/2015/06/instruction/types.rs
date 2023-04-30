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
    pub fn iter_area(&self) -> impl Iterator<Item = Coordinate> {
        let CoordinatePair(from_coordinate, to_coordinate) = self;

        (from_coordinate.x..=to_coordinate.x)
            .zip(from_coordinate.y..=to_coordinate.y)
            .map(|(x, y)| Coordinate { x, y })
    }
}

// impl IntoIterator for CoordinatePair {
//     type Item = Coordinate;

//     type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

//     fn into_iter(self) -> Self::IntoIter {
//         let CoordinatePair(from_coordinate, to_coordinate) = coordinate_pair;

//         (from_coordinate.x..=to_coordinate.x)
//             .zip(from_coordinate.y..=to_coordinate.y)
//             .map(|(x, y)| Coordinate { x, y })
//     }
// }
