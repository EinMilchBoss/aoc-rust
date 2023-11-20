use util::aoc;

#[derive(Debug)]
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
    let mut sensors = vec![];
    for line in input.lines() {
        let parts = line.split(' ');
        let mut parts = parts.skip(2);
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        let location = parse_location(x, y);
        let mut parts = parts.skip(4);
        let beacon = parse_beacon(parts.next().unwrap(), parts.next().unwrap());
        sensors.push(Sensor { location, beacon });
    }

    dbg!(&sensors);

    0
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
