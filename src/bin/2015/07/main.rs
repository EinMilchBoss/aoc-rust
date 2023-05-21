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

fn is_command_executable(wires: &HashMap<Wire, Signal>, command_inputs: &[CommandInput]) -> bool {
    for command_input in command_inputs {
        if let CommandInput::Wire(wire) = command_input {
            if wires.contains_key(&wire) {
                return false;
            }
        }
    }
    return true;
}

// fn signal_from_command_input(wires: &HashMap<Wire, Signal>, command_input: CommandInput) -> Signal {
//     match command_input {
//         CommandInput::Signal(signal) => signal,
//         CommandInput::Wire(wire) => *wires.get(&wire).unwrap(),
//     }
// }

// fn execute_instructions(wires: &mut HashMap<Wire, Signal>, instructions: Vec<Instruction>) {
//     if instructions.is_empty() {
//         return;
//     }

//     let mut unprocessed_instructions = Vec::new();
//     for instruction in instructions {
//         match instruction.command {
//             Command::Set(new_signal) => {
//                 let signal = signal_from_command_input(wires, new_signal);
//                 *wires.entry(instruction.destination).or_insert(signal) = signal;
//             }
//             Command::And(left, right) => {
//                 if !is_command_executable(wires, &vec![left, right]) {
//                     unprocessed_instructions.push(instruction);
//                     continue;
//                 }

//                 let left = match left {
//                     CommandInput::Signal(signal) => signal.0,
//                     CommandInput::Wire(wire) => wires.get(&wire).unwrap().0,
//                 };

//                 let right = match right {
//                     CommandInput::Signal(signal) => signal.0,
//                     CommandInput::Wire(wire) => wires.get(&wire).unwrap().0,
//                 };

//                 let new_signal = Signal(left & right);
//                 *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
//             }
//             Command::Or(left, right) => {
//                 if !is_command_executable(wires, &vec![left, right]) {
//                     unprocessed_instructions.push(instruction);
//                     continue;
//                 }

//                 let left = match left {
//                     CommandInput::Signal(signal) => signal.0,
//                     CommandInput::Wire(wire) => wires.get(&wire).unwrap().0,
//                 };

//                 let right = match right {
//                     CommandInput::Signal(signal) => signal.0,
//                     CommandInput::Wire(wire) => wires.get(&wire).unwrap().0,
//                 };

//                 let new_signal = Signal(left | right);
//                 *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
//             }
//             Command::LeftShift { value, shift } => match value {
//                 CommandInput::Signal(signal) => {
//                     let new_signal = Signal(signal.0 << shift);
//                     *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
//                 }
//                 CommandInput::Wire(wire) => {
//                     let wire = wires.get(&wire);
//                     if let Some(wire) = wire {
//                         let new_signal = Signal(wire.0 << shift);
//                         *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
//                     } else {
//                         unprocessed_instructions.push(instruction);
//                     }
//                 }
//             },
//             Command::RightShift { value, shift } => match value {
//                 CommandInput::Signal(signal) => {
//                     let new_signal = Signal(signal.0 << shift);
//                     *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
//                 }
//                 CommandInput::Wire(wire) => {
//                     let wire = wires.get(&wire);
//                     if let Some(wire) = wire {
//                         let new_signal = Signal(wire.0 << shift);
//                         *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
//                     } else {
//                         unprocessed_instructions.push(instruction);
//                     }
//                 }
//             },
//             Command::Not(input) => match input {
//                 CommandInput::Signal(signal) => {
//                     let new_signal = Signal(!signal.0);
//                     *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
//                 }
//                 CommandInput::Wire(wire) => {
//                     let wire = wires.get(&wire);
//                     if let Some(wire) = wire {
//                         let new_signal = Signal(!wire.0);
//                         *wires.entry(instruction.destination).or_insert(new_signal) = new_signal;
//                     } else {
//                         unprocessed_instructions.push(instruction);
//                     }
//                 }
//             },
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

struct WireSignalStates {
    wire_to_signal: HashMap<Wire, Signal>,
}

impl WireSignalStates {
    pub fn new() -> WireSignalStates {
        WireSignalStates {
            wire_to_signal: HashMap::new(),
        }
    }

    pub fn process_instructions(&mut self, instructions: &[Instruction]) {
        fn next(wire_signal_states: &mut WireSignalStates, instructions: Vec<Instruction>) {
            let unprocessed_instructions: Vec<_> = instructions
                .into_iter()
                .filter(|instruction| !wire_signal_states.try_execute_instruction(instruction))
                .collect();
            if !unprocessed_instructions.is_empty() {
                next(wire_signal_states, unprocessed_instructions);
            }
        }
        next(self, instructions.to_vec());
    }

    pub fn wire_signal_state(&self, wire: &Wire) -> Option<Signal> {
        self.wire_to_signal.get(wire).cloned()
    }

    fn try_execute_instruction(&mut self, instruction: &Instruction) -> bool {
        match &instruction.command {
            Command::Set(command_input) => {
                self.try_execute_command_set(command_input, &instruction.destination)
            }
            Command::And(left_command_input, right_command_input) => self.try_execute_command_and(
                left_command_input,
                right_command_input,
                &instruction.destination,
            ),
            Command::Or(left_command_input, right_command_input) => self.try_execute_command_or(
                left_command_input,
                right_command_input,
                &instruction.destination,
            ),
            Command::LeftShift {
                value: command_input,
                shift,
            } => {
                self.try_execute_command_left_shift(command_input, *shift, &instruction.destination)
            }
            Command::RightShift {
                value: command_input,
                shift,
            } => self.try_execute_command_right_shift(
                command_input,
                *shift,
                &instruction.destination,
            ),
            Command::Not(command_input) => {
                self.try_execute_command_not(command_input, &instruction.destination)
            }
        }
    }

    fn try_execute_command_set(
        &mut self,
        command_input: &CommandInput,
        destination: &Wire,
    ) -> bool {
        match self.signal(command_input) {
            Some(signal) => {
                *self
                    .wire_to_signal
                    .entry(destination.clone())
                    .or_insert(signal) = signal;
                true
            }
            _ => false,
        }
    }

    fn try_execute_command_and(
        &mut self,
        left_command_input: &CommandInput,
        right_command_input: &CommandInput,
        destination: &Wire,
    ) -> bool {
        match (
            self.signal(left_command_input),
            self.signal(right_command_input),
        ) {
            (Some(left_signal), Some(right_signal)) => {
                let new_signal = Signal(left_signal.0 & right_signal.0);
                *self
                    .wire_to_signal
                    .entry(destination.clone())
                    .or_insert(new_signal) = new_signal;
                true
            }
            _ => false,
        }
    }

    fn try_execute_command_or(
        &mut self,
        left_command_input: &CommandInput,
        right_command_input: &CommandInput,
        destination: &Wire,
    ) -> bool {
        match (
            self.signal(left_command_input),
            self.signal(right_command_input),
        ) {
            (Some(left_signal), Some(right_signal)) => {
                let new_signal = Signal(left_signal.0 | right_signal.0);
                *self
                    .wire_to_signal
                    .entry(destination.clone())
                    .or_insert(new_signal) = new_signal;
                true
            }
            _ => false,
        }
    }

    fn try_execute_command_left_shift(
        &mut self,
        command_input: &CommandInput,
        shift: SignalType,
        destination: &Wire,
    ) -> bool {
        match self.signal(command_input) {
            Some(signal) => {
                let new_signal = Signal(signal.0 << shift);
                *self
                    .wire_to_signal
                    .entry(destination.clone())
                    .or_insert(new_signal) = new_signal;
                true
            }
            _ => false,
        }
    }

    fn try_execute_command_right_shift(
        &mut self,
        command_input: &CommandInput,
        shift: SignalType,
        destination: &Wire,
    ) -> bool {
        match self.signal(command_input) {
            Some(signal) => {
                let new_signal = Signal(signal.0 >> shift);
                *self
                    .wire_to_signal
                    .entry(destination.clone())
                    .or_insert(new_signal) = new_signal;
                true
            }
            _ => false,
        }
    }

    fn try_execute_command_not(
        &mut self,
        command_input: &CommandInput,
        destination: &Wire,
    ) -> bool {
        match self.signal(command_input) {
            Some(signal) => {
                let new_signal = Signal(!signal.0);
                *self
                    .wire_to_signal
                    .entry(destination.clone())
                    .or_insert(new_signal) = new_signal;
                true
            }
            _ => false,
        }
    }

    fn signal(&self, command_input: &CommandInput) -> Option<Signal> {
        match command_input {
            CommandInput::Signal(signal) => Some(*signal),
            CommandInput::Wire(wire) => self.wire_to_signal.get(wire).cloned(),
        }
    }
}

fn solve_first(input: &str) -> String {
    let instructions = parse_instructions(input).unwrap();

    let mut wire_signal_states = WireSignalStates::new();
    wire_signal_states.process_instructions(&instructions);

    // go through every instruction
    // execute instruction
    // execute bezieht sich immer auf eine hashmap an Wires und Signals (HashMap<Wire, Signal>)

    wire_signal_states
        .wire_signal_state(&Wire::from_id("a"))
        .expect("Wire `a` does not exist even after processing all instructions.")
        .0
        .to_string()

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
