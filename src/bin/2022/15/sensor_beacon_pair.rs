use crate::coordinate::Coordinate;

#[derive(Debug, Clone, Copy)]
pub struct SensorBeaconPair {
    pub sensor: Coordinate,
    pub beacon: Coordinate,
}

impl SensorBeaconPair {
    pub fn from_input_line(line: &str) -> Self {
        let parts = line.split(' ');

        let mut skipped = parts.skip(2);
        let sensor = parse_location(skipped.next().unwrap(), skipped.next().unwrap());

        let mut skipped = skipped.skip(4);
        let beacon = parse_beacon(skipped.next().unwrap(), skipped.next().unwrap());

        Self { sensor, beacon }
    }

    pub fn manhattan_between(&self) -> u32 {
        self.sensor.manhattan(self.beacon)
    }
}

fn parse_location(x: &str, y: &str) -> Coordinate {
    let x = x[2..x.len() - 1].parse().unwrap();
    let y = y[2..y.len() - 1].parse().unwrap();

    Coordinate { x, y }
}

fn parse_beacon(x: &str, y: &str) -> Coordinate {
    let x = x[2..x.len() - 1].parse().unwrap();
    let y = y[2..].parse().unwrap();

    Coordinate { x, y }
}
