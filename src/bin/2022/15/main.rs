use util::aoc;

fn part_one(input: &str) -> i32 {
    0
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

    println!("Part one:\n{result}", result = one.run());
    println!("Part two:\n{result}", result = two.run());

    Ok(())
}
