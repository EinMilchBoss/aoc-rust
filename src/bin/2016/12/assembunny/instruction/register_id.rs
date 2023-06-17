#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegisterId {
    A,
    B,
    C,
    D,
}

impl From<char> for RegisterId {
    fn from(value: char) -> Self {
        match value {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            _ => panic!("Cannot parse `{}` to `RegisterId`.", value),
        }
    }
}

#[cfg(test)]
mod register_id_tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(RegisterId::A, 'a')]
    #[case(RegisterId::B, 'b')]
    #[case(RegisterId::C, 'c')]
    #[case(RegisterId::D, 'd')]
    fn trait_from_from_test_valid_register_id(#[case] expected: RegisterId, #[case] actual: char) {
        assert_eq!(expected, actual.into());
    }

    #[test]
    #[should_panic]
    fn trait_from_from_test_invalid_register_id() {
        let _: RegisterId = 'x'.into();
    }
}
