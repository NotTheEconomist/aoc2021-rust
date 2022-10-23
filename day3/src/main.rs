use std::cmp::Ordering;

const INPUT: &str = include_str!("input.txt");

fn parse_input_as_binary(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|line| u16::from_str_radix(line, 2).expect("could not parse as binary"))
        .collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum BinaryDigit {
    Zero,
    One,
}

impl BinaryDigit {
    fn not(self) -> Self {
        match self {
            Self::One => Self::Zero,
            Self::Zero => Self::One,
        }
    }
    fn digit(self) -> u16 {
        match self {
            Self::One => 1,
            Self::Zero => 0,
        }
    }
}

impl Default for BinaryDigit {
    fn default() -> Self {
        BinaryDigit::Zero
    }
}

impl PartialEq<u16> for BinaryDigit {
    fn eq(&self, other: &u16) -> bool {
        matches!(other, 1)
    }
}

impl TryFrom<u16> for BinaryDigit {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BinaryDigit::Zero),
            1 => Ok(BinaryDigit::One),
            _ => Err(format!("can't parse {} as binary digit", value)),
        }
    }
}

impl From<BinaryDigit> for u16 {
    fn from(digit: BinaryDigit) -> Self {
        match digit {
            BinaryDigit::Zero => 0,
            BinaryDigit::One => 1,
        }
    }
}

impl From<bool> for BinaryDigit {
    fn from(bit: bool) -> Self {
        match bit {
            false => Self::Zero,
            true => Self::One,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
struct BinaryDigitCounter {
    ones: usize,
    zeroes: usize,
}

impl BinaryDigitCounter {
    fn majority(&self) -> BinaryDigit {
        match self.ones.cmp(&self.zeroes) {
            Ordering::Greater => BinaryDigit::One,
            Ordering::Less => BinaryDigit::Zero,
            Ordering::Equal => panic!("We got as many ones as zeroes -- input must be wrong!"),
        }
    }
    fn majority_or(&self, equal_case: BinaryDigit) -> BinaryDigit {
        match self.ones.cmp(&self.zeroes) {
            Ordering::Greater => BinaryDigit::One,
            Ordering::Less => BinaryDigit::Zero,
            Ordering::Equal => equal_case,
        }
    }
    fn digit(&self) -> u16 {
        self.majority().digit()
    }
    fn not_digit(&self) -> u16 {
        self.majority().not().digit()
    }
}

#[derive(Debug, PartialEq)]
struct BinaryDigitCounters {
    size: usize,
    counters: [BinaryDigitCounter; 16],
}

impl Default for BinaryDigitCounters {
    fn default() -> Self {
        Self {
            size: 16,
            counters: Default::default(),
        }
    }
}

impl BinaryDigitCounters {
    fn get_sigbit(mut i: u16) -> usize {
        let mut sigbit = 0;
        while i > 0 {
            sigbit += 1;
            i >>= 1;
        }
        sigbit
    }
    fn with_size(size: usize) -> Self {
        Self {
            size,
            counters: Default::default(),
        }
    }
    fn with_bits(self, bitses: &Vec<u16>) -> Self {
        let mut new = Self {
            size: self.size,
            counters: Default::default(),
        };

        for bits in bitses {
            new.push(bits)
        }

        new
    }
    fn from_bits(bitses: &Vec<u16>) -> Self {
        let max_size = bitses.iter().fold(0, |acc, bits| {
            let sigbit = Self::get_sigbit(*bits);
            if sigbit > acc {
                sigbit
            } else {
                acc
            }
        });
        Self::with_size(max_size).with_bits(bitses)
    }
    fn push(&mut self, bits: &u16) {
        for (i, mut bdc) in (0..self.size).zip(self.counters.iter_mut().rev()) {
            let mask = 1 << i;
            let bit = (bits & mask) >> i;
            match bit.try_into().expect("Could not parse as binarydigit") {
                BinaryDigit::Zero => bdc.zeroes += 1,
                BinaryDigit::One => bdc.ones += 1,
            };
        }
    }

    fn iter(&self) -> std::slice::Iter<BinaryDigitCounter> {
        self.counters[16 - self.size..].iter()
    }

    fn collect_majority(&self) -> u16 {
        self.iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, bdc)| acc | bdc.digit() << (i as u16))
    }

    fn collect_minority(&self) -> u16 {
        self.iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, bdc)| acc | bdc.not_digit() << (i as u16))
    }
}

enum CalculationType {
    Gamma,
    Epsilon,
    Oxygen,
    Carbondioxide,
}

fn calculate(input: &str, calculation: CalculationType) -> u32 {
    match calculation {
        CalculationType::Gamma => calculate_gamma(input),
        CalculationType::Epsilon => calculate_epsilon(input),
        CalculationType::Oxygen => calculate_oxygen(input),
        CalculationType::Carbondioxide => calculate_carbondioxide(input),
    }
}

fn calculate_gamma(input: &str) -> u32 {
    let bitcounter = BinaryDigitCounters::from_bits(&parse_input_as_binary(input));
    bitcounter.collect_majority() as u32
}
fn calculate_epsilon(input: &str) -> u32 {
    let bitcounter = BinaryDigitCounters::from_bits(&parse_input_as_binary(input));
    bitcounter.collect_minority() as u32
}
fn calculate_oxygen(input: &str) -> u32 {
    let mut candidates = parse_input_as_binary(input);
    let bitlength = input.lines().next().unwrap().chars().count();
    for i in 0..bitlength {
        if candidates.len() == 1 {
            break;
        }
        let bitcounters = BinaryDigitCounters::with_size(bitlength).with_bits(&candidates);
        let mask = 1 << (bitlength - i - 1);
        let desired = bitcounters
            .iter()
            .nth(i)
            .expect("bad digit number")
            .majority_or(BinaryDigit::One)
            .digit()
            << (bitlength - i - 1);
        candidates.retain(|&n| n & mask == desired);
    }
    assert!(candidates.len() == 1);
    candidates.first().unwrap().to_owned() as u32
}
fn calculate_carbondioxide(input: &str) -> u32 {
    let mut candidates = parse_input_as_binary(input);
    let bitlength = input.lines().next().unwrap().chars().count();
    for i in 0..bitlength {
        if candidates.len() == 1 {
            break;
        }
        let bitcounters = BinaryDigitCounters::with_size(bitlength).with_bits(&candidates);
        let mask = 1 << (bitlength - 1 - i);
        let desired = bitcounters
            .iter()
            .nth(i)
            .expect("bad digit number")
            .majority_or(BinaryDigit::One)
            .not()
            .digit()
            << (bitlength - 1 - i);
        candidates.retain(|&n| n & mask == desired);
    }
    assert!(candidates.len() == 1);
    candidates.first().unwrap().to_owned() as u32
}

fn main() {
    let gamma = calculate(INPUT, CalculationType::Gamma);
    let epsilon = calculate(INPUT, CalculationType::Epsilon);
    println!("part1: {}", (gamma as u32) * (epsilon as u32));

    // O2 generator rating filters across the majority bitfilter
    let oxygen = calculate(INPUT, CalculationType::Oxygen);
    let carbondioxide = calculate(INPUT, CalculationType::Carbondioxide);

    println!("part2: {}", (oxygen as u32) * (carbondioxide as u32));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("test_input.txt");

    mod integration {
        use super::*;

        #[test]
        fn test_gamma() {
            let want = 0b10110; // 22
            let gamma = calculate_gamma(TEST_INPUT);
            assert_eq!(want, gamma);
        }

        #[test]
        fn test_epsilon() {
            let want = 0b01001; // 9
            let epsilon = calculate_epsilon(TEST_INPUT);
            assert_eq!(want, epsilon);
        }

        #[test]
        #[ignore]
        fn test_oxygen() {
            let want = 0b10111; // 23
            let oxygen = calculate_oxygen(TEST_INPUT);
            assert_eq!(want, oxygen);
        }

        #[test]
        #[ignore]
        fn test_carbondioxide() {
            let want = 0b01010; // 10
            let carbondioxide = calculate_carbondioxide(TEST_INPUT);
            assert_eq!(want, carbondioxide);
        }
    }

    #[test]
    fn test_day1_integration() {
        let test_input = parse_input_as_binary(TEST_INPUT);
        let initial = BinaryDigitCounters::from_bits(&test_input);
        let gamma = initial.collect_majority();
        let epsilon = initial.collect_minority();
        assert_eq!(gamma * epsilon, 198);
    }

    #[test]
    fn test_parse_input() {
        let want = [
            0b00100u16, 0b11110u16, 0b10110u16, 0b10111u16, 0b10101u16, 0b01111u16, 0b00111u16,
            0b11100u16, 0b10000u16, 0b11001u16, 0b00010u16, 0b01010u16,
        ]
        .to_vec();
        assert_eq!(parse_input_as_binary(TEST_INPUT), want)
    }

    #[test]
    fn test_binarydigit_collects() {
        let bits: u16 = 0b1111111111111111;
        let initial = BinaryDigitCounters::from_bits(&vec![bits]);
        assert_eq!(initial.collect_majority(), bits);
        assert_eq!(initial.collect_minority(), 0);

        let bits: u16 = 0b1001001111100100;
        let initial = BinaryDigitCounters::from_bits(&vec![bits]);
        assert_eq!(initial.collect_majority(), bits);
        assert_eq!(initial.collect_minority(), !bits);
    }

    #[test]
    fn test_binarydigit_from_bits() {
        let bits: u16 = 0b1111111111111111;
        let mut initial = BinaryDigitCounters::from_bits(&vec![bits]);
        let want = BinaryDigitCounters {
            size: 16,
            counters: [
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
                BinaryDigitCounter { ones: 1, zeroes: 0 },
            ],
        };
        assert_eq!(initial, want);
        initial.push(&0b1111);
        let want = BinaryDigitCounters {
            size: 16,
            counters: [
                BinaryDigitCounter { ones: 1, zeroes: 1 },
                BinaryDigitCounter { ones: 1, zeroes: 1 },
                BinaryDigitCounter { ones: 1, zeroes: 1 },
                BinaryDigitCounter { ones: 1, zeroes: 1 },
                BinaryDigitCounter { ones: 1, zeroes: 1 },
                BinaryDigitCounter { ones: 1, zeroes: 1 },
                BinaryDigitCounter { ones: 1, zeroes: 1 },
                BinaryDigitCounter { ones: 1, zeroes: 1 },
                BinaryDigitCounter { ones: 1, zeroes: 1 },
                BinaryDigitCounter { ones: 1, zeroes: 1 },
                BinaryDigitCounter { ones: 1, zeroes: 1 },
                BinaryDigitCounter { ones: 1, zeroes: 1 },
                BinaryDigitCounter { ones: 2, zeroes: 0 },
                BinaryDigitCounter { ones: 2, zeroes: 0 },
                BinaryDigitCounter { ones: 2, zeroes: 0 },
                BinaryDigitCounter { ones: 2, zeroes: 0 },
            ],
        };
        assert_eq!(initial, want);
    }
}
