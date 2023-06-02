use crate::triangle::TriangleCollection;

pub fn parse_input_horizontal(input: &str) -> TriangleCollection {
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::triangle::Triangle;

    #[rstest]
    fn parse_input_horizontal_test_ok() {
        let actual = ["  676  739   39", "  890   40  865", "    2  735  297"].join("\n");
        let expected = TriangleCollection::new(vec![
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
        ]);

        assert_eq!(expected, parse_input_horizontal(&actual));
    }

    #[test]
    #[should_panic]
    fn parse_input_horizontal_test_err_not_enough_sides_per_line() {
        let input = "  1    2";

        parse_input_horizontal(input);
    }

    #[test]
    #[should_panic]
    fn parse_input_horizontal_test_err_no_valid_number() {
        let input = "  1    A    3";

        parse_input_horizontal(input);
    }
}
