use std::collections::HashSet;
use std::str::FromStr;

const START: Point2D = Point2D::from_cartesian(0, 0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point2D {
    x: isize,
    y: isize,
}

impl Point2D {
    const fn from_cartesian(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn manhattan_distance_to(&self, other: Point2D) -> usize {
        let dx = (self.x - other.x).abs() as usize;
        let dy = (self.y - other.y).abs() as usize;
        dx + dy
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Steps(usize);

#[derive(Debug, PartialEq)]
enum Instruction {
    Left(Steps),
    Right(Steps),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut string_iter = string.chars();

        let first_char = string_iter.nth(0).ok_or(())?;
        let steps = string_iter
            .collect::<String>()
            .parse::<usize>()
            .map_err(|_| ())?;

        match first_char {
            'L' => Ok(Self::Left(Steps(steps))),
            'R' => Ok(Self::Right(Steps(steps))),
            _ => Err(()),
        }
    }
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

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&mut self) {
        *self = match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        };
    }

    fn turn_right(&mut self) {
        *self = match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        };
    }
}

#[derive(Debug, PartialEq)]
struct PathFollowingPlayer {
    position: Point2D,
    direction: Direction,
}

impl PathFollowingPlayer {
    fn at_start() -> Self {
        Self {
            position: Point2D::from_cartesian(0, 0),
            direction: Direction::North,
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Left(steps) => {
                self.direction.turn_left();
                self.walk(*steps);
            }
            Instruction::Right(steps) => {
                self.direction.turn_right();
                self.walk(*steps);
            }
        };
    }

    fn walk(&mut self, steps: Steps) {
        let steps = steps.0 as isize;
        match self.direction {
            Direction::North => self.position.y += steps,
            Direction::East => self.position.x += steps,
            Direction::South => self.position.y -= steps,
            Direction::West => self.position.x -= steps,
        };
    }

    fn distance_from_start(&self) -> usize {
        self.position.manhattan_distance_to(START)
    }
}

#[derive(Debug, PartialEq)]
struct PathRememberingPlayer {
    position: Point2D,
    direction: Direction,
    visited_positions: HashSet<Point2D>,
}

impl PathRememberingPlayer {
    fn at_start() -> Self {
        Self {
            position: Point2D::from_cartesian(0, 0),
            direction: Direction::North,
            visited_positions: HashSet::new(),
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Left(steps) => {
                self.direction.turn_left();
                self.walk(*steps);
            }
            Instruction::Right(steps) => {
                self.direction.turn_right();
                self.walk(*steps);
            }
        };
    }

    fn walk(&mut self, steps: Steps) {
        let steps = steps.0 as isize;
        match self.direction {
            Direction::North => self.position.y += steps,
            Direction::East => self.position.x += steps,
            Direction::South => self.position.y -= steps,
            Direction::West => self.position.x -= steps,
        };
    }

    fn distance_from_start(&self) -> usize {
        self.position.manhattan_distance_to(START)
    }
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

fn main() {
    let instructions = parse_instructions(INPUT);

    println!("Part 1: \"{}\"", part_1(&instructions));
    // println!("Part 2: \"{}\"", part_2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_remembering_player() {
        let input = "R1, L2, R3, R4, R5";
        assert_eq!(3, part_1(&parse_instructions(input)));

        let input = "L1, R2, L3, L4, L5";
        assert_eq!(3, part_1(&parse_instructions(input)));
    }

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
    fn path_following_player_distance_from_start_test() {
        let mut player = PathFollowingPlayer::at_start();
        player.position = Point2D::from_cartesian(5, -10);

        assert_eq!(15, player.distance_from_start());
    }

    #[test]
    fn direction_turn_left_test() {
        let mut direction = Direction::North;

        direction.turn_left();
        assert_eq!(Direction::West, direction);

        direction.turn_left();
        assert_eq!(Direction::South, direction);

        direction.turn_left();
        assert_eq!(Direction::East, direction);

        direction.turn_left();
        assert_eq!(Direction::North, direction);
    }

    #[test]
    fn direction_turn_right_test() {
        let mut direction = Direction::North;

        direction.turn_right();
        assert_eq!(Direction::East, direction);

        direction.turn_right();
        assert_eq!(Direction::South, direction);

        direction.turn_right();
        assert_eq!(Direction::West, direction);

        direction.turn_right();
        assert_eq!(Direction::North, direction);
    }

    #[test]
    fn path_following_player_walk_test() {
        let mut player = PathFollowingPlayer::at_start();

        player.direction = Direction::North;
        player.walk(Steps(5));
        assert_eq!(player.position, Point2D::from_cartesian(0, 5));

        player.direction = Direction::East;
        player.walk(Steps(5));
        assert_eq!(player.position, Point2D::from_cartesian(5, 5));

        player.direction = Direction::South;
        player.walk(Steps(5));
        assert_eq!(player.position, Point2D::from_cartesian(5, 0));

        player.direction = Direction::West;
        player.walk(Steps(5));
        assert_eq!(player.position, Point2D::from_cartesian(0, 0));
    }

    #[test]
    fn path_following_player_execute_instruction_test_left() {
        let mut player = PathFollowingPlayer::at_start();
        let instruction = Instruction::Left(Steps(10));
        let expected = PathFollowingPlayer {
            position: Point2D { x: -10, y: 0 },
            direction: Direction::West,
        };

        player.execute_instruction(&instruction);

        assert_eq!(expected, player);
    }

    #[test]
    fn path_following_player_execute_instruction_test_right() {
        let mut player = PathFollowingPlayer::at_start();
        let instruction = Instruction::Right(Steps(10));
        let expected = PathFollowingPlayer {
            position: Point2D { x: 10, y: 0 },
            direction: Direction::East,
        };

        player.execute_instruction(&instruction);

        assert_eq!(expected, player);
    }

    #[test]
    fn path_following_player_at_start_test() {
        let expected = PathFollowingPlayer {
            position: Point2D { x: 0, y: 0 },
            direction: Direction::North,
        };

        assert_eq!(expected, PathFollowingPlayer::at_start());
    }

    #[test]
    #[should_panic]
    fn parse_instructions_test_failure() {
        let input = "L12_R34";

        parse_instructions(input);
    }

    #[test]
    fn parse_instructions_test_success() {
        let input = "L12, R34";
        let expected = vec![Instruction::Left(Steps(12)), Instruction::Right(Steps(34))];

        assert_eq!(expected, parse_instructions(input));
    }

    #[test]
    fn instruction_trait_from_str_test_err_steps() {
        let negative_steps = "L-12";
        let gibberish_steps = "L_*!";
        let no_steps = "L";

        assert_eq!(Err(()), negative_steps.parse::<Instruction>());
        assert_eq!(Err(()), gibberish_steps.parse::<Instruction>());
        assert_eq!(Err(()), no_steps.parse::<Instruction>());
    }

    #[test]
    fn instruction_trait_from_str_test_err_direction() {
        let invalid_direction = "*12";

        assert_eq!(Err(()), invalid_direction.parse::<Instruction>());
    }

    #[test]
    fn instruction_trait_from_str_test_ok() {
        let left = "L12";
        let right = "R34";

        assert_eq!(Ok(Instruction::Left(Steps(12))), left.parse());
        assert_eq!(Ok(Instruction::Right(Steps(34))), right.parse());
    }

    #[test]
    fn point_from_cartesian_test() {
        let point = Point2D::from_cartesian(5, -10);

        assert_eq!(5, point.x);
        assert_eq!(-10, point.y);
    }

    #[test]
    fn manhattan_distance_test_neg2pos_neg2neg() {
        let start_point = Point2D::from_cartesian(-12, -2);
        let end_point = Point2D::from_cartesian(5, -10);

        assert_eq!(25, start_point.manhattan_distance_to(end_point));
    }

    #[test]
    fn manhattan_distance_test_pos2neg_pos2pos() {
        let start_point = Point2D::from_cartesian(12, 2);
        let end_point = Point2D::from_cartesian(-5, 10);

        assert_eq!(25, start_point.manhattan_distance_to(end_point));
    }
}

const INPUT: &str = "R2, L3, R2, R4, L2, L1, R2, R4, R1, L4, L5, R5, R5, R2, R2, R1, L2, L3, L2, L1, R3, L5, R187, R1, R4, L1, R5, L3, L4, R50, L4, R2, R70, L3, L2, R4, R3, R194, L3, L4, L4, L3, L4, R4, R5, L1, L5, L4, R1, L2, R4, L5, L3, R4, L5, L5, R5, R3, R5, L2, L4, R4, L1, R3, R1, L1, L2, R2, R2, L3, R3, R2, R5, R2, R5, L3, R2, L5, R1, R2, R2, L4, L5, L1, L4, R4, R3, R1, R2, L1, L2, R4, R5, L2, R3, L4, L5, L5, L4, R4, L2, R1, R1, L2, L3, L2, R2, L4, R3, R2, L1, L3, L2, L4, L4, R2, L3, L3, R2, L4, L3, R4, R3, L2, L1, L4, R4, R2, L4, L4, L5, L1, R2, L5, L2, L3, R2, L2";
