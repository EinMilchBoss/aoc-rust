use std::collections::HashMap;

use crate::parser::{Command, CommandInput, Instruction, Signal, SignalType, Wire};

pub struct WireSignalStates {
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
