use itertools::Itertools;

use crate::triangle::TriangleCollection;

pub fn parse_input_vertical(input: &str) -> TriangleCollection {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .flat_map(|lines| {
            let sides_of_triangles: [_; 3] = lines
                .map(super::parse_sides)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|_| panic!("Amount of lines is not divisible by 3."));

            rows_to_columns(sides_of_triangles)
        })
        .collect()
}

fn rows_to_columns(rows: [[usize; 3]; 3]) -> [[usize; 3]; 3] {
    let mut columns = [[0; 3]; 3];
    for (column_index, column) in columns.iter_mut().enumerate() {
        *column = sides_of_column(rows, column_index);
    }
    columns
}

fn sides_of_column(rows: [[usize; 3]; 3], column_index: usize) -> [usize; 3] {
    let mut column = [0; 3];
    for (index, row) in rows.into_iter().enumerate() {
        column[index] = row[column_index];
    }
    column
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
