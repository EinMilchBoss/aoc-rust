#[derive(Debug)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug)]
pub struct CoordinatePair(pub Coordinate, pub Coordinate);

#[derive(Debug)]
pub enum Command {
    On,
    Off,
    Toggle,
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

#[derive(Debug)]
pub struct Instruction {
    pub command: Command,
    pub coordinate_pair: CoordinatePair,
}
