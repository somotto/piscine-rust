pub fn spell(n: u64) -> String {
    match n {
        0 => String::from("zero"),
        1_000_000 => String::from("one million"),
        1..=19 => {
            match n {
                1 => "one",
                2 => "two",
                3 => "three",
                4 => "four",
                5 => "five",
                6 => "six",
                7 => "seven",
                8 => "eight",
                9 => "nine",
                10 => "ten",
                11 => "eleven",
                12 => "twelve",
                13 => "thirteen",
                14 => "fourteen",
                15 => "fifteen",
                16 => "sixteen",
                17 => "seventeen",
                18 => "eighteen",
                19 => "nineteen",
                _ => unreachable!(),
            }.to_string()
        },
        20..=99 => {
            let tens = n / 10;
            let ones = n % 10;
            
            let tens_word = match tens {
                2 => "twenty",
                3 => "thirty",
                4 => "forty",
                5 => "fifty",
                6 => "sixty",
                7 => "seventy",
                8 => "eighty",
                9 => "ninety",
                _ => unreachable!(),
            };
            
            match ones {
                0 => tens_word.to_string(),
                _ => format!("{}-{}", tens_word, spell(ones)),
            }
        },
        100..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            
            match remainder {
                0 => format!("{} hundred", spell(hundreds)),
                _ => format!("{} hundred {}", spell(hundreds), spell(remainder)),
            }
        },
        1_000..=999_999 => {
            let thousands = n / 1000;
            let remainder = n % 1000;
            
            match remainder {
                0 => format!("{} thousand", spell(thousands)),
                _ => format!("{} thousand {}", spell(thousands), spell(remainder)),
            }
        },
        _ => unreachable!()
    }
}