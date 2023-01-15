use core::panic;

use util::std::*;

const YEAR: Year = Year("2022");
const DAY: Day = Day("05");

type CrateStack = Vec<char>;

struct Instruction {
    amount: u8,
    from: u8,
    to: u8,
}

fn crate_stack_amount(last_line: &str) -> usize {
    last_line
        .chars()
        .filter(|char| char.is_ascii_digit())
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize
}

fn parse_crate_stacks(crate_lines: &Vec<&str>, amount: usize) -> Vec<CrateStack> {
    let mut crate_stacks = vec![CrateStack::new(); amount];
    let crate_char_indices = (0..amount)
        .into_iter()
        .map(|it| 4 * it + 1)
        .collect::<Vec<_>>();

    for line in crate_lines.iter().rev() {
        let crate_chars = line
            .chars()
            .enumerate()
            .filter(|(index, _)| crate_char_indices.contains(index))
            .map(|(_, val)| val);

        for (stack, crate_char) in crate_chars.enumerate() {
            if crate_char.is_whitespace() {
                continue;
            }
            crate_stacks[stack].push(crate_char);
        }
    }

    crate_stacks
}

fn parse_instructions(instruction_lines: Vec<&str>) -> Vec<Instruction> {
    instruction_lines
        .iter()
        .map(|line| {
            let splitted = line
                .split_whitespace()
                .filter_map(|it| it.parse::<u8>().ok())
                .collect::<Vec<_>>();
            if let [amount, from, to] = splitted[..] {
                Instruction { amount, from, to }
            } else {
                panic!("Instruction parsing failed.")
            }
        })
        .collect::<Vec<_>>()
}

fn peek_crates(crate_stacks: &Vec<CrateStack>) -> String {
    crate_stacks
        .iter()
        .map(|it| it.last().unwrap())
        .collect::<String>()
}

fn solve<F>(input: &str, f: F) -> String
where
    F: Fn(&mut Vec<CrateStack>, &Instruction),
{
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let mut crate_lines = parts[0].lines().collect::<Vec<_>>();
    let instruction_lines = parts[1].lines().collect::<Vec<_>>();

    let last_line = crate_lines.pop().unwrap();
    let crate_stack_amount = crate_stack_amount(last_line);
    let mut crate_stacks = parse_crate_stacks(&crate_lines, crate_stack_amount);

    let instructions = parse_instructions(instruction_lines);

    instructions.iter().for_each(|it| f(&mut crate_stacks, it));

    peek_crates(&crate_stacks)
}

fn solve_first(input: &str) -> String {
    solve(input, |crate_stacks, instruction| {
        let Instruction { amount, from, to } = *instruction;
        for _ in 0..amount {
            let popped = crate_stacks[from as usize - 1].pop().unwrap();
            crate_stacks[to as usize - 1].push(popped);
        }
    })
}

fn solve_second(input: &str) -> String {
    solve(input, |crate_stacks, instruction| {
        let Instruction { amount, from, to } = *instruction;

        let from_crate_stack = &crate_stacks[from as usize - 1];
        let truncated_length = from_crate_stack.len() - amount as usize;
        let removed = from_crate_stack[truncated_length..].to_vec();

        crate_stacks[to as usize - 1].extend(removed);
        crate_stacks[from as usize - 1].truncate(truncated_length);
    })
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!("First: Expected {} found {}.", "CMZ", solve_first(&example));
        println!(
            "Second: Expected {} found {}.",
            "MCD",
            solve_second(&example)
        );
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
