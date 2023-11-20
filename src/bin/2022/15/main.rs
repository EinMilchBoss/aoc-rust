use std::{collections::HashSet, ops::RangeInclusive};

use util::aoc;

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn manhattan(self, other: Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug)]
struct Sensor {
    location: Coordinate,
    beacon: Coordinate,
}

const LINE: i32 = 2_000_000;

fn part_one(input: &str) -> usize {
    let mut sensors = Vec::new();
    let mut xs = HashSet::new();
    let mut subs = HashSet::new();
    for line in input.lines() {
        let parts = line.split(' ');

        let mut skipped = parts.skip(2);
        let location = parse_location(skipped.next().unwrap(), skipped.next().unwrap());

        let mut skipped = skipped.skip(4);
        let beacon = parse_beacon(skipped.next().unwrap(), skipped.next().unwrap());

        let sensor = Sensor { location, beacon };
        sensors.push(sensor);

        let manhattan = location.manhattan(beacon);
        if let Some(vals) = x_values(location, manhattan) {
            vals.into_iter().for_each(|x| {
                xs.insert(x);
            })
        }
    }

    for sensor in sensors {
        if sensor.beacon.y == LINE && xs.contains(&sensor.beacon.x) {
            subs.insert(sensor.beacon.x);
        }

        if sensor.location.y == LINE && xs.contains(&sensor.location.x) {
            subs.insert(sensor.location.x);
        }
    }

    xs.len() - subs.len()
}

fn x_values(location: Coordinate, manhattan: u32) -> Option<RangeInclusive<i32>> {
    let min = location.y - manhattan as i32;
    let max = location.y + manhattan as i32;
    if !(min..=max).contains(&LINE) {
        return None;
    }

    let dy = location.y.abs_diff(LINE);
    let side = (manhattan - dy) as i32;
    Some(location.x - side..=location.x + side)
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

fn part_two(input: &str) -> i32 {
    0
}

fn main() -> Result<(), String> {
    let inputs = aoc::Inputs::read(2022, 15).map_err(|err| err.to_string())?;
    let one = aoc::Part::one(&inputs, part_one);
    let two = aoc::Part::two(&inputs, part_two);

    print!("{protocol}\n\n", protocol = one.test_protocol(26));
    print!("{protocol}\n\n", protocol = two.test_protocol(0));

    println!("Part one:\n{result}", result = one.run());
    //println!("Part two:\n{result}", result = two.run());

    Ok(())
}
