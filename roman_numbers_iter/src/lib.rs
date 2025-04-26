use std::iter::Iterator;

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.to_u32();
        let next_value = value + 1;
        let next_roman = RomanNumber::from(next_value);
        Some(next_roman)
    }
}
