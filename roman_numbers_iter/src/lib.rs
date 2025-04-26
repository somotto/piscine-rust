mod roman_numbers;
use roman_numbers::RomanNumber;

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.to_u32() + 1;
        *self = RomanNumber::from(value);
        Some(self.clone())
    }
}