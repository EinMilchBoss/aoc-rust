use util::std::*;

const YEAR: Year = Year("2022");
const DAY: Day = Day("04");

fn solve_first(input: &str) -> String {
    let x = 3..=5;
    // x.all(|part| y.contains(part))
    // len() for the amount?
    // filtering out non matching

    let parsed: Vec<_> = input
        .lines()
        .map(|line| { 
            let mut ranges = line
                .split(',')
                .map(|range| {
                    let mut sides  = range
                        .split('-')
                        .map(|side| side.parse::<u8>().unwrap());
                    let start = sides.next().unwrap();
                    let end = sides.next().unwrap();
                    start..=end
                });
            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .collect();

    let overlapping: Vec<_> = parsed
        .iter()
        .cloned()
        .filter(|(first, second)| {
            let first_overlaps = first.start() <= second.start() && first.end() >= second.end();
            let second_overlaps = second.start() <= first.start() && second.end() >= first.end();
            first_overlaps || second_overlaps
        })
        .collect();

    //dbg!(&overlapping);


    
    overlapping.len().to_string()
}

fn solve_second(input: &str) -> String {
    let parsed: Vec<_> = input
        .lines()
        .map(|line| { 
            let mut ranges = line
                .split(',')
                .map(|range| {
                    let mut sides  = range
                        .split('-')
                        .map(|side| side.parse::<u8>().unwrap());
                    let start = sides.next().unwrap();
                    let end = sides.next().unwrap();
                    start..=end
                });
            (ranges.next().unwrap(), ranges.next().unwrap())
        })
        .collect();

    let overlapping: Vec<_> = parsed
        .iter()
        .cloned()
        .filter(|range| {
            let (mut first, mut second) = range.clone();
            let first_overlaps = second.any(|part| first.contains(&part));
            let second_overlaps = first.any(|part| second.contains(&part));
            first_overlaps || second_overlaps
        })
        .collect();

    //dbg!(&overlapping);


    
    overlapping.len().to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!(
            "First: Expected {} found {}.",
            2,
            solve_first(&example)
        );
        println!(
            "Second: Expected {} found {}.",
            45_000,
            solve_second(&example)
        );
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
