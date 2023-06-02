use itertools::Itertools;

use crate::triangle::TriangleCollection;

pub fn parse_input_vertical(input: &str) -> TriangleCollection {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .flat_map(|lines| {
            let triangles_sides: [_; 3] = lines
                .map(super::parse_sides)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|_| panic!("Amount of lines is not divisible by 3."));

            rotate_sides_of_triangles(triangles_sides)
        })
        .collect()
}

fn rotate_sides_of_triangles(
    sides_of_triangles: [(usize, usize, usize); 3],
) -> [(usize, usize, usize); 3] {
    [
        (
            sides_of_triangles[0].0,
            sides_of_triangles[1].0,
            sides_of_triangles[2].0,
        ),
        (
            sides_of_triangles[0].1,
            sides_of_triangles[1].1,
            sides_of_triangles[2].1,
        ),
        (
            sides_of_triangles[0].2,
            sides_of_triangles[1].2,
            sides_of_triangles[2].2,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
    use crate::triangle::Triangle;

    #[rstest]
    fn parse_input_vertical_test_ok() {
        let actual = ["  676  739   39", "  890   40  865", "    2  735  297"].join("\n");
        let expected = TriangleCollection::new(vec![
            Triangle {
                a: 676,
                b: 890,
                c: 2,
            },
            Triangle {
                a: 739,
                b: 40,
                c: 735,
            },
            Triangle {
                a: 39,
                b: 865,
                c: 297,
            },
        ]);

        assert_eq!(expected, parse_input_vertical(&actual));
    }

    #[test]
    #[should_panic]
    fn parse_input_vertical_test_err_not_enough_sides_per_line() {
        let input = ["    1    2", "    4    5    6", "    7    8    9"].join("\n");

        parse_input_vertical(&input);
    }

    #[test]
    #[should_panic]
    fn parse_input_vertical_test_err_no_valid_number() {
        let input = ["    1    2    *", "    4    5    6", "    7    8    9"].join("\n");

        parse_input_vertical(&input);
    }

    #[test]
    #[should_panic]
    fn parse_input_vertical_test_err_number_of_lines_not_divisable_by_3() {
        let input = ["    1    2    3", "    4    5    6"].join("\n");

        parse_input_vertical(&input);
    }
}
