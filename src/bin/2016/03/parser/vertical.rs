use itertools::Itertools;

use crate::triangle::TriangleCollection;

pub fn parse_input_vertical(input: &str) -> TriangleCollection {
    // lines() -> chunks(3)

    input
        .lines()
        .chunks(3)
        .into_iter()
        .flat_map(|lines| {
            let sides: Vec<_> = lines.map(parse_sides).collect();
            let mut triangles = [(0, 0, 0); 3];
            if let &[a1, a2, a3] = sides[0].as_slice() {
                triangles[0].0 = a1;
                triangles[1].0 = a2;
                triangles[2].0 = a3;
            }
            if let &[b1, b2, b3] = sides[1].as_slice() {
                triangles[0].1 = b1;
                triangles[1].1 = b2;
                triangles[2].1 = b3;
            }
            if let &[c1, c2, c3] = sides[2].as_slice() {
                triangles[0].2 = c1;
                triangles[1].2 = c2;
                triangles[2].2 = c3;
            }
            triangles
        })
        .collect()
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
