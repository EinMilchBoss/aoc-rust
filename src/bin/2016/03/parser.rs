mod horizontal;
mod vertical;

pub use horizontal::parse_input_horizontal;
pub use vertical::parse_input_vertical;

fn parse_sides(line: &str) -> [usize; 3] {
    let sides: Vec<usize> = [&line[2..=4], &line[7..=9], &line[12..]]
        .into_iter()
        .map(parse_side)
        .collect();

    match sides.as_slice() {
        &[a, b, c] => [a, b, c],
        _ => panic!(
            "There are less than 3 numbers in one line of the input. Content: \"{}\", found: {:?}.",
            line, sides
        ),
    }
}

fn parse_side(side: &str) -> usize {
    String::from(side.trim())
        .parse()
        .unwrap_or_else(|_| panic!("Could not parse string \"{}\".", side))
}
