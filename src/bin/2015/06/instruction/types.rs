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
pub struct CoordinatePair(pub Coordinate, pub Coordinate);

impl CoordinatePair {
    pub fn area(self) -> CoordinatePairArea {
        CoordinatePairArea::new(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}

pub struct CoordinatePairArea {
    coordinate_pair: CoordinatePair,
    current: Option<Coordinate>,
    is_first: bool,
}

impl CoordinatePairArea {
    fn new(coordinate_pair: CoordinatePair) -> Self {
        let CoordinatePair(first, _) = coordinate_pair;
        CoordinatePairArea {
            coordinate_pair,
            current: Some(first),
            is_first: true,
        }
    }

    fn update_current_coordinate(&mut self) {
        if self.current.is_some() {
            if self.is_end_of_row() {
                self.start_at_next_row();
            } else {
                self.move_right();
            }

            if self.is_done() {
                self.current = None;
            }
        }
    }

    fn is_end_of_row(&self) -> bool {
        let CoordinatePair(_, highest) = self.coordinate_pair;
        let last = self.safely_unwrap_current();
        last.x >= highest.x
    }

    fn start_at_next_row(&mut self) {
        let CoordinatePair(lowest, _) = self.coordinate_pair;
        let last = self.safely_unwrap_current();
        self.current = Some(Coordinate {
            x: lowest.x,
            y: last.y + 1,
        });
    }

    fn move_right(&mut self) {
        let last = self.safely_unwrap_current();
        self.current = Some(Coordinate {
            x: last.x + 1,
            y: last.y,
        })
    }

    fn safely_unwrap_current(&self) -> Coordinate {
        self.current.expect("Current element should not be `None`.")
    }

    fn is_done(&self) -> bool {
        let CoordinatePair(_, highest) = self.coordinate_pair;
        match self.current {
            Some(current) => current.y > highest.y,
            None => true,
        }
    }
}

impl Iterator for CoordinatePairArea {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
        } else {
            self.update_current_coordinate();
        }
        self.current
    }
}
