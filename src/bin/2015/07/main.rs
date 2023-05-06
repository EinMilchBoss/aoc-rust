mod parser;

use std::collections::HashMap;

use parser::*;
use util::std::*;

const YEAR: Year = Year("2015");
const DAY: Day = Day("07");

// go through every instruction
// if corresponding wires to all necessary input signals are given
// execute instruction
// else
// add to unprocessed instructions
// call for unprocessed instructions recursively

// fn execute_instructions(wires: &mut HashMap<Wire, Signal>, instructions: Vec<Instruction>) {
//     if instructions.is_empty() {
//         return;
//     }

//     dbg!(wires.len());

//     let mut unprocessed_instructions = Vec::new();
//     for instruction in instructions {
//         dbg!(&instruction);
//         match instruction.command {
//             Command::Set(new_signal) => {
//                 *wires.entry(instruction.destination).or_insert(new_signal) = new_signal
//             }
//             Command::And(ref left, ref right) => {
//                 let left = wires.get(left);
//                 let right = wires.get(right);
//                 if let (Some(left), Some(right)) = (left, right) {
//                     let new_signal = Signal(left.0 & right.0);
//                     *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
//                 } else {
//                     unprocessed_instructions.push(instruction);
//                 }
//             }
//             Command::Or(ref left, ref right) => {
//                 let left = wires.get(left);
//                 let right = wires.get(right);
//                 if let (Some(left), Some(right)) = (left, right) {
//                     let new_signal = Signal(left.0 | right.0);
//                     *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
//                 } else {
//                     unprocessed_instructions.push(instruction);
//                 }
//             }
//             Command::LeftShift(ref wire, ref shift) => {
//                 let wire = wires.get(wire);
//                 if let Some(wire) = wire {
//                     let new_signal = Signal(wire.0 << shift);
//                     *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
//                 } else {
//                     unprocessed_instructions.push(instruction);
//                 }
//             }
//             Command::RightShift(ref wire, shift) => {
//                 let wire = wires.get(wire);
//                 if let Some(wire) = wire {
//                     let new_signal = Signal(wire.0 >> shift);
//                     *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
//                 } else {
//                     unprocessed_instructions.push(instruction);
//                 }
//             }
//             Command::Not(ref wire) => {
//                 let wire = wires.get(wire);
//                 if let Some(wire) = wire {
//                     let new_signal = Signal(!wire.0);
//                     *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
//                 } else {
//                     unprocessed_instructions.push(instruction);
//                 }
//             }
//         };
//     }

//     execute_instructions(wires, unprocessed_instructions);
// }

// fn execute_set_instructions(
//     wires: &mut HashMap<Wire, Signal>,
//     instructions: Vec<Instruction>,
// ) -> Vec<Instruction> {
//     let mut unprocessed = Vec::new();
//     for instruction in instructions {
//         if let Command::Set(new_signal) = instruction.command {
//             dbg!(&instruction);
//             *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
//         } else {
//             unprocessed.push(instruction);
//         }
//     }
//     unprocessed
// }

fn solve_first(input: &str) -> String {
    let instructions = parse_instructions(input).unwrap();
    dbg!(instructions.len());
    dbg!(&instructions);

    "".to_string()

    // // first execute all set commands
    // // check if all inputs to a gate are already in "wires" map
    // // remove executed instructions

    // let mut wires: HashMap<Wire, Signal> = HashMap::new();

    // let instructions = parse_instructions(input).unwrap();
    // dbg!(instructions.len());
    // dbg!(&instructions);
    // let remaining_instructions = execute_set_instructions(&mut wires, instructions);

    // execute_instructions(&mut wires, remaining_instructions);

    // // instructions.iter().for_each(|instruction| {
    // //     if let Command::Set(new_signal) = instruction.command {
    // //         *wires
    // //             .entry(instruction.destination.clone())
    // //             .or_insert(new_signal) = new_signal
    // //     }
    // // });

    // // let mut unprocessed_instructions = Vec::new();

    // // for instruction in instructions {
    // //     match instruction.command {
    // //         Command::Set(new_signal) => {
    // //             *wires.entry(instruction.destination).or_insert(new_signal) = new_signal
    // //         }
    // //         Command::And(ref left, ref right) => {
    // //             let left = wires.get(left);
    // //             let right = wires.get(right);
    // //             if let (Some(left), Some(right)) = (left, right) {
    // //                 let new_signal = Signal(left.0 & right.0);
    // //                 *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
    // //             } else {
    // //                 unprocessed_instructions.push(instruction);
    // //             }
    // //         }
    // //         Command::Or(ref left, ref right) => {
    // //             let left = wires.get(left);
    // //             let right = wires.get(right);
    // //             if let (Some(left), Some(right)) = (left, right) {
    // //                 let new_signal = Signal(left.0 | right.0);
    // //                 *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
    // //             } else {
    // //                 unprocessed_instructions.push(instruction);
    // //             }
    // //         }
    // //         Command::LeftShift(ref wire, ref shift) => {
    // //             let wire = wires.get(wire);
    // //             if let Some(wire) = wire {
    // //                 let new_signal = Signal(wire.0 << shift);
    // //                 *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
    // //             } else {
    // //                 unprocessed_instructions.push(instruction);
    // //             }
    // //         }
    // //         Command::RightShift(ref wire, shift) => {
    // //             let wire = wires.get(wire);
    // //             if let Some(wire) = wire {
    // //                 let new_signal = Signal(wire.0 >> shift);
    // //                 *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
    // //             } else {
    // //                 unprocessed_instructions.push(instruction);
    // //             }
    // //         }
    // //         Command::Not(ref wire) => {
    // //             let wire = wires.get(wire);
    // //             if let Some(wire) = wire {
    // //                 let new_signal = Signal(!wire.0);
    // //                 *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
    // //             } else {
    // //                 unprocessed_instructions.push(instruction);
    // //             }
    // //         }
    // //     };
    // // }

    // dbg!(&wires);

    // wires.get(&Wire("a".to_string())).unwrap().0.to_string()
}

fn solve_second(input: &str) -> String {
    "".to_string()
}

fn main() {
    let example = read_file(InputFile::Example, YEAR, DAY);
    let input = read_file(InputFile::Actual, YEAR, DAY);

    if let Some(example) = example {
        println!("First: Expected {} found {}.", 65412, solve_first(&example));
        println!("Second: Expected {} found {}.", 0, solve_second(&example));
    }

    if let Some(input) = input {
        println!("First: {}", solve_first(&input));
        println!("Second: {}", solve_second(&input));
    }
}
