use std::collections::HashSet;

use util::aoc;

mod coordinate;
mod sensor_beacon_pair;

use coordinate::*;
use sensor_beacon_pair::*;

/*
As my AOC "framework" doesn't contain an option to pass arguments, such as `LINE`,
this value has to be changed manually between testing and actually running for real.
*/
const LINE_IMPOSSIBLE_VALUES: i32 = 2_000_000;

fn part_one(input: &str) -> usize {
    let mut impossibles = HashSet::new();
    let mut ignorables = HashSet::new();

    for line in input.lines() {
        let pair = SensorBeaconPair::from_input_line(line);

        if pair.beacon.y == LINE_IMPOSSIBLE_VALUES {
            ignorables.insert(pair.beacon);
        }

        if let Some(values) = impossibles_for_line(pair) {
            for value in values {
                impossibles.insert(value);
            }
        }
    }

    for Coordinate { x, .. } in &ignorables {
        impossibles.remove(x);
    }

    impossibles.len()
}

fn impossibles_for_line(pair: SensorBeaconPair) -> Option<impl Iterator<Item = i32>> {
    let SensorBeaconPair {
        sensor: Coordinate { x, y },
        ..
    } = pair;
    let manhattan = pair.manhattan_between();

    let min = y - manhattan as i32;
    let max = y + manhattan as i32;
    if !(min..=max).contains(&LINE_IMPOSSIBLE_VALUES) {
        return None;
    }

    let dy = y.abs_diff(LINE_IMPOSSIBLE_VALUES);
    let side = (manhattan - dy) as i32;
    Some(x - side..=x + side)
}

fn part_two(_: &str) -> i32 {
    0
}

fn main() -> Result<(), String> {
    let inputs = aoc::Inputs::read(2022, 15).map_err(|err| err.to_string())?;
    let one = aoc::Part::one(&inputs, part_one);
    let two = aoc::Part::two(&inputs, part_two);

    print!("{protocol}\n\n", protocol = one.test_protocol(26));
    print!("{protocol}\n\n", protocol = two.test_protocol(0));

    println!("Part one:\n{result}", result = one.run());
    println!("Part two:\n{result}", result = two.run());

    Ok(())
}
