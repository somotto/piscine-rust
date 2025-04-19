#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => panic!("Invalid value for single RomanDigit"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        let mut result = Vec::new();

        let roman_values = [
            (1000, vec![RomanDigit::M]),
            (900, vec![RomanDigit::C, RomanDigit::M]),
            (500, vec![RomanDigit::D]),
            (400, vec![RomanDigit::C, RomanDigit::D]),
            (100, vec![RomanDigit::C]),
            (90, vec![RomanDigit::X, RomanDigit::C]),
            (50, vec![RomanDigit::L]),
            (40, vec![RomanDigit::X, RomanDigit::L]),
            (10, vec![RomanDigit::X]),
            (9, vec![RomanDigit::I, RomanDigit::X]),
            (5, vec![RomanDigit::V]),
            (4, vec![RomanDigit::I, RomanDigit::V]),
            (1, vec![RomanDigit::I]),
        ];

        for (value, digits) in roman_values.iter() {
            while num >= *value {
                num -= *value;
                result.extend_from_slice(digits);
            }
        }

        RomanNumber(result)
    }
}
