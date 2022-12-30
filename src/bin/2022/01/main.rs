use util::std::*;

const YEAR: Year = Year("2022");
const DAY: Day = Day("01");

fn solve(input: &str) -> Vec<i32> {
    input.split("\n\n")
        .map(|elf: &str| 
            elf.lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum::<i32>()
        )
        .collect()
}

fn solve_first(input: &str) -> String {
    solve(input)
        .iter()
        .max()
        .unwrap()
        .to_string()
}

fn solve_second(input: &str) -> String {
    let mut sums: Vec<i32> = solve(input);
    sums.sort_unstable_by(|a, b| b.cmp(a));
    sums[..3].iter().sum::<i32>().to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        assert_eq!(24_000.to_string(), solve_first(&example), "First example");
        assert_eq!(45_000.to_string(), solve_second(&example), "Second example");
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}