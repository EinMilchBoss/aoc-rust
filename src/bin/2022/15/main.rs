use std::{collections::HashSet, ops::RangeInclusive};

use util::aoc;

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    location: Coordinate,
    beacon: Coordinate,
}

const LINE: i32 = 10;

fn part_one(input: &str) -> usize {
    // parse Sensors and Beacons
    // distance: Manhattan
    // get coordinate area until beacon
    // - increment if y = 2_000_000
    // - THIS CAN BE OPTIMIZED BY ONLY LOOKING AT Y 2_000_000
    let mut ss: Vec<_> = input
        .lines()
        .map(|line| {
            let parts = line.split(' ');

            let mut skipped = parts.skip(2);
            let location = parse_location(skipped.next().unwrap(), skipped.next().unwrap());

            let mut skipped = skipped.skip(4);
            let beacon = parse_beacon(skipped.next().unwrap(), skipped.next().unwrap());

            Sensor { location, beacon }
        })
        .collect();

    // dbg!(&ss);

    // ss.push(Sensor {
    //     location: Coordinate { x: 0, y: 2_000_000 },
    //     beacon: Coordinate { x: 2, y: 2_000_000 },
    // });

    // let ms: Vec<_> = ss.iter().map(|s| manhattan(s.location, s.beacon)).collect();

    // dbg!(&ms);

    let mut x_vals: HashSet<i32> = HashSet::new();
    for s in &ss {
        if let Some(vals) = x_values(s.location, manhattan(s.location, s.beacon)) {
            vals.into_iter().for_each(|x| {
                x_vals.insert(x);
            })
        }
    }
    //dbg!(&x_vals);

    let mut sub: HashSet<i32> = HashSet::new();
    for s in &ss {
        if s.beacon.y == LINE && x_vals.contains(&s.beacon.x) {
            //dbg!(&s);
            sub.insert(s.beacon.x);
        }

        if s.location.y == LINE && x_vals.contains(&s.location.x) {
            //dbg!(&s);
            sub.insert(s.location.x);
        }
    }
    //println!("SUB {sub:?}");

    x_vals.len() - sub.len()
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

fn manhattan(from: Coordinate, to: Coordinate) -> u32 {
    to.x.abs_diff(from.x) + to.y.abs_diff(from.y)
}

fn parse_location(x: &str, y: &str) -> Coordinate {
    // dbg!(x);
    // dbg!(y);

    let x = x[2..x.len() - 1].parse().unwrap();
    let y = y[2..y.len() - 1].parse().unwrap();

    Coordinate { x, y }
}

fn parse_beacon(x: &str, y: &str) -> Coordinate {
    // dbg!(x);
    // dbg!(y);

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
