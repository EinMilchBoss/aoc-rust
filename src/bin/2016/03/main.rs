mod parser;
mod triangle;

use util::std::*;

use parser::parse_input_horizontal;
use parser::parse_input_vertical;

fn main() {
    let input = read_file(InputFile::Actual, Year("2016"), Day("03"))
        .expect("Input file could not be read.");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    parse_input_horizontal(input)
        .iter()
        .filter(|triangle| triangle.is_valid())
        .count()
}

fn part_2(input: &str) -> usize {
    parse_input_vertical(input)
        .iter()
        .filter(|triangle| triangle.is_valid())
        .count()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn raw_input() -> String {
        let input_lines = ["  676  739   39", "  890   40  865", "    2  735  297"];
        input_lines.join("\n")
    }

    #[rstest]
    fn part_1_test(raw_input: String) {
        assert_eq!(1, part_1(&raw_input));
    }

    #[rstest]
    fn part_2_test(raw_input: String) {
        assert_eq!(1, part_2(&raw_input));
    }
}
