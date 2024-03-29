use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct TriangleCollection(Vec<Triangle>);

impl TriangleCollection {
    #[cfg(test)]
    pub fn new(triangles: Vec<Triangle>) -> Self {
        Self(triangles)
    }
}

impl Deref for TriangleCollection {
    type Target = Vec<Triangle>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<[usize; 3]> for TriangleCollection {
    fn from_iter<T: IntoIterator<Item = [usize; 3]>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|[a, b, c]| Triangle { a, b, c })
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Triangle {
    pub fn is_valid(&self) -> bool {
        let is_c_valid = (self.a + self.b) > self.c;
        let is_b_valid = (self.a + self.c) > self.b;
        let is_a_valid = (self.b + self.c) > self.a;
        is_a_valid && is_b_valid && is_c_valid
    }
}

#[cfg(test)]
mod triangle_collection_tests {
    use super::*;

    #[test]
    fn from_iterator_trait_from_iter_test() {
        let input: [[usize; 3]; 3] = [[1, 2, 3], [4, 5, 6], [10, 420, 5]];
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
}

#[cfg(test)]
mod triangle_tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(true, Triangle {a: 5, b: 5, c: 2})]
    #[case(true, Triangle {a: 5, b: 2, c: 5})]
    #[case(true, Triangle {a: 2, b: 5, c: 5})]
    #[case(false, Triangle {a: 5, b: 3, c: 2})]
    #[case(false, Triangle {a: 5, b: 2, c: 2})]
    fn is_valid_test(#[case] expected: bool, #[case] triangle: Triangle) {
        assert_eq!(expected, triangle.is_valid());
    }
}
