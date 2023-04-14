use util::std::*;

use itertools::Itertools;

const YEAR: Year = Year("2015");
const DAY: Day = Day("02");

struct RectangularPrism(u32, u32, u32);

impl TryFrom<&[u32]> for RectangularPrism {
    type Error = ();

    fn try_from(value: &[u32]) -> Result<Self, Self::Error> {
        if let &[l, w, h] = value {
            Ok(RectangularPrism(l, w, h))
        } else {
            Err(())
        }
    }
}

impl RectangularPrism {
    fn wrapping_paper_size(&self) -> u32 {
        let RectangularPrism(l, w, h) = self;
        let sides = [l * w, w * h, h * l];
        let smallest_side = sides.into_iter().min().unwrap();

        sides.into_iter().fold(0u32, |acc, e| acc + 2 * e) + smallest_side
    }

    fn ribbon_length(&self) -> u32 {
        let RectangularPrism(l, w, h) = self;
        let edges = [l, w, h];
        let smallest_edges = edges.into_iter().sorted().take(2).collect_vec();

        let present = smallest_edges.into_iter().fold(0u32, |acc, e| acc + 2 * e);
        let bow: u32 = edges.into_iter().product();

        present + bow
    }
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split('x')
        .map(|number| {
            number
                .parse()
                .unwrap_or_else(|_| panic!("String {} could not be parsed to number.", number))
        })
        .collect_vec()
}

fn solve_first(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            RectangularPrism::try_from(parse_line(line).as_slice())
                .unwrap_or_else(|_| panic!("Could not parse line {}.", line))
                .wrapping_paper_size()
        })
        .sum::<u32>()
        .to_string()
}

fn solve_second(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            RectangularPrism::try_from(parse_line(line).as_slice())
                .unwrap_or_else(|_| panic!("Could not parse line {}.", line))
                .ribbon_length()
        })
        .sum::<u32>()
        .to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!("First: Expected {} found {}.", 58, solve_first(&example));
        println!("Second: Expected {} found {}.", 34, solve_second(&example));
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
