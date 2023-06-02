use crate::triangle::TriangleCollection;

pub fn parse_input_horizontal(input: &str) -> TriangleCollection {
    input.lines().map(super::parse_sides).collect()
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
