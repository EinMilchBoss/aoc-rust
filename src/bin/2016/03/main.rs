use std::ops::Deref;

use util::std::*;

#[derive(Debug, Clone, PartialEq)]
struct TriangleCollection(Vec<Triangle>);

impl Deref for TriangleCollection {
    type Target = Vec<Triangle>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle {
    fn is_valid(&self) -> bool {
        let is_c_valid = (self.a + self.b) > self.c;
        let is_b_valid = (self.a + self.c) > self.b;
        let is_a_valid = (self.b + self.c) > self.a;
        is_a_valid && is_b_valid && is_c_valid
    }
}

impl FromIterator<(usize, usize, usize)> for TriangleCollection {
    fn from_iter<T: IntoIterator<Item = (usize, usize, usize)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(a, b, c)| Triangle { a, b, c })
                .collect(),
        )
    }
}

fn main() {
    let input = read_file(InputFile::Actual, Year("2016"), Day("03"))
        .expect("Input file could not be read.");
    let input = parse_input(&input);

    println!("Part 1: {}", part_1(&input));
    // println!("Part 2: {}", part_2(&input));
}

fn parse_input(input: &str) -> TriangleCollection {
    input.lines().map(|line| {
        let sides = parse_sides(line);
        if let &[a, b, c] = sides.as_slice() {
            (a, b, c)
        } else {
            panic!("There are less than 3 numbers in one line of the input. Content: \"{}\", found: {:?}.", line, sides)
        }
    }).collect()
}

fn parse_sides(line: &str) -> Vec<usize> {
    [&line[2..=4], &line[7..=9], &line[12..]]
        .into_iter()
        .map(parse_side)
        .collect()
}

fn parse_side(side: &str) -> usize {
    String::from(side.trim())
        .parse()
        .unwrap_or_else(|_| panic!("Could not parse string \"{}\".", side))
}

fn part_1(input: &[Triangle]) -> usize {
    input.iter().filter(|triangle| triangle.is_valid()).count()
}

fn part_2(input: &[Triangle]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[rstest]
    #[case(true, Triangle {a: 5, b: 5, c: 2})]
    #[case(true, Triangle {a: 5, b: 2, c: 5})]
    #[case(true, Triangle {a: 2, b: 5, c: 5})]
    #[case(false, Triangle {a: 5, b: 3, c: 2})]
    #[case(false, Triangle {a: 5, b: 2, c: 2})]
    fn triangle_is_valid_test(#[case] expected: bool, #[case] triangle: Triangle) {
        assert_eq!(expected, triangle.is_valid());
    }

    #[test]
    fn triangle_trait_from_iterator_test() {
        let input: [(usize, usize, usize); 3] = [(1, 2, 3), (4, 5, 6), (10, 420, 5)];
        let expected = TriangleCollection(vec![
            Triangle { a: 1, b: 2, c: 3 },
            Triangle { a: 4, b: 5, c: 6 },
            Triangle {
                a: 10,
                b: 420,
                c: 5,
            },
        ]);

        assert_eq!(expected, input.into_iter().collect());
    }

    #[fixture]
    fn raw_input() -> String {
        let input_lines = ["  676  739   39", "  890   40  865", "    2  735  297"];
        input_lines.join("\n")
    }

    #[fixture]
    fn parsed_input() -> TriangleCollection {
        TriangleCollection(vec![
            Triangle {
                a: 676,
                b: 739,
                c: 39,
            },
            Triangle {
                a: 890,
                b: 40,
                c: 865,
            },
            Triangle {
                a: 2,
                b: 735,
                c: 297,
            },
        ])
    }

    #[rstest]
    fn parse_input_test_ok(parsed_input: TriangleCollection, raw_input: String) {
        assert_eq!(parsed_input, parse_input(&raw_input));
    }

    #[test]
    #[should_panic]
    fn parse_input_test_err_not_enough_sides_per_line() {
        let input = "  1    2";

        parse_input(input);
    }

    #[test]
    #[should_panic]
    fn parse_input_test_err_no_valid_number() {
        let input = "  1    A    3";

        parse_input(input);
    }

    #[rstest]
    fn part_1_test(parsed_input: TriangleCollection) {
        assert_eq!(1, part_1(&parsed_input));
    }
}
