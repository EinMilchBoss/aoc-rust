mod direction;
mod instruction;
mod player;
mod point;

use instruction::Instruction;
use player::{PathFollowingPlayer, PathRememberingPlayer};

fn main() {
    let instructions = parse_instructions(INPUT);

    println!("Part 1: \"{}\"", part_1(&instructions));
    // println!("Part 2: \"{}\"", part_2(&instructions));
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .split(", ")
        .map(|string| {
            string.parse().unwrap_or_else(|_| {
                panic!(
                    "String \"{}\" could not be parsed to `Instruction`.",
                    string
                );
            })
        })
        .collect()
}

fn part_1(instructions: &[Instruction]) -> usize {
    let mut player = PathFollowingPlayer::at_start();
    for instruction in instructions {
        player.execute_instruction(instruction);
    }
    player.distance_from_start()
}

fn part_2(instructions: &[Instruction]) -> usize {
    let mut player = PathRememberingPlayer::at_start();
    for instruction in instructions {
        player.execute_instruction(instruction);
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::instruction::Steps;

    use super::*;

    #[test]
    fn part_1_test() {
        let input = "R1, L2, R3, R4, R5";
        assert_eq!(3, part_1(&parse_instructions(input)));

        let input = "L1, R2, L3, L4, L5";
        assert_eq!(3, part_1(&parse_instructions(input)));
    }

    #[test]
    fn part_2_test() {
        let input = "R4, R8, R4, R8";
        assert_eq!(4, part_2(&parse_instructions(input)));

        let input = "L4, L8, L4, L8";
        assert_eq!(4, part_2(&parse_instructions(input)));
    }

    #[test]
    fn parse_instructions_test_success() {
        let input = "L12, R34";
        let expected = vec![Instruction::Left(Steps(12)), Instruction::Right(Steps(34))];

        assert_eq!(expected, parse_instructions(input));
    }

    #[test]
    #[should_panic]
    fn parse_instructions_test_failure() {
        let input = "L12_R34";

        parse_instructions(input);
    }
}

const INPUT: &str = "R2, L3, R2, R4, L2, L1, R2, R4, R1, L4, L5, R5, R5, R2, R2, R1, L2, L3, L2, L1, R3, L5, R187, R1, R4, L1, R5, L3, L4, R50, L4, R2, R70, L3, L2, R4, R3, R194, L3, L4, L4, L3, L4, R4, R5, L1, L5, L4, R1, L2, R4, L5, L3, R4, L5, L5, R5, R3, R5, L2, L4, R4, L1, R3, R1, L1, L2, R2, R2, L3, R3, R2, R5, R2, R5, L3, R2, L5, R1, R2, R2, L4, L5, L1, L4, R4, R3, R1, R2, L1, L2, R4, R5, L2, R3, L4, L5, L5, L4, R4, L2, R1, R1, L2, L3, L2, R2, L4, R3, R2, L1, L3, L2, L4, L4, R2, L3, L3, R2, L4, L3, R4, R3, L2, L1, L4, R4, R2, L4, L4, L5, L1, R2, L5, L2, L3, R2, L2";
