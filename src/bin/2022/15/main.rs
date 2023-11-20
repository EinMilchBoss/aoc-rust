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

fn part_one(input: &str) -> i32 {
    // parse Sensors and Beacons
    // distance: Manhattan
    // get coordinate area until beacon
    // - increment if y = 2_000_000
    // - THIS CAN BE OPTIMIZED BY ONLY LOOKING AT Y 2_000_000
    let ss: Vec<_> = input
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

    dbg!(&ss);

    let ms: Vec<_> = ss.iter().map(|s| manhattan(s.location, s.beacon)).collect();

    dbg!(&ms);

    0
}

fn manhattan(from: Coordinate, to: Coordinate) -> u32 {
    to.x.abs_diff(from.x) + to.y.abs_diff(from.y)
}

fn parse_location(x: &str, y: &str) -> Coordinate {
    dbg!(x);
    dbg!(y);

    let x = x[2..x.len() - 1].parse().unwrap();
    let y = y[2..y.len() - 1].parse().unwrap();

    Coordinate { x, y }
}

fn parse_beacon(x: &str, y: &str) -> Coordinate {
    dbg!(x);
    dbg!(y);

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

    print!("{protocol}\n\n", protocol = one.test_protocol(10));
    print!("{protocol}\n\n", protocol = two.test_protocol(20));

    //println!("Part one:\n{result}", result = one.run());
    //println!("Part two:\n{result}", result = two.run());

    Ok(())
}
