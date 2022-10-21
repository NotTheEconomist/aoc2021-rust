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
    margin: u32,
    majority: BinaryDigit,
}

impl BinaryDigitCounter {
    fn digit(&self) -> u16 {
        self.majority.digit()
    }
    fn not_digit(&self) -> u16 {
        self.majority.not().digit()
    }
}

#[derive(Default, Debug, PartialEq)]
struct BinaryDigitCounters([BinaryDigitCounter; 16]);

impl BinaryDigitCounters {
    fn from_bits(bitses: &Vec<u16>) -> Self {
        let mut acc: Self = Default::default();
        for bits in bitses {
            acc.push(bits)
        }
        acc
    }
    fn push(&mut self, bits: &u16) {
        for (i, mut bdc) in self.0.iter_mut().enumerate() {
            let mask = 1 << (16 - i - 1);
            let bit = (bits & mask) >> (16 - i - 1);
            if bdc.majority == bit {
                // Increase the margin by one
                bdc.margin += 1;
            } else if bdc.margin == 0 {
                // Flip which way the majority goes
                bdc.margin = 1;
                bdc.majority = bit.try_into().expect("can't parse digit as binary")
            } else {
                // Reduce the margin by one
                bdc.margin -= 1;
            }
        }
    }

    fn truncate_to(&mut self, size: usize) {
        for i in 0..(16 - size) {
            self.0[i] = BinaryDigitCounter::default();
        }
    }

    fn collect_majority(&self) -> u16 {
        let mut acc = 0;
        for (i, bdc) in self.0.iter().enumerate() {
            acc |= bdc.digit() << (16 - i - 1);
        }
        acc
    }

    fn collect_minority(&self) -> u16 {
        let mut acc = 0;
        for (i, bdc) in self.0.iter().enumerate() {
            let digit: u16 = if bdc.margin == 0 { 0 } else { bdc.not_digit() };
            acc |= digit << (16 - i - 1);
        }
        acc
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
    let mut bitcounter = BinaryDigitCounters::from_bits(&parse_input_as_binary(input));
    let bitlength = input.lines().next().unwrap().chars().count();
    bitcounter.truncate_to(bitlength);
    bitcounter.collect_majority() as u32
}
fn calculate_epsilon(input: &str) -> u32 {
    let mut bitcounter = BinaryDigitCounters::from_bits(&parse_input_as_binary(input));
    let bitlength = input.lines().next().unwrap().chars().count();
    bitcounter.truncate_to(bitlength);
    bitcounter.collect_minority() as u32
}
fn calculate_oxygen(input: &str) -> u32 {
    let mut candidates = parse_input_as_binary(input);
    let bitlength = input.lines().next().unwrap().chars().count();
    for i in 0..bitlength {
        let mut bitcounter = BinaryDigitCounters::from_bits(&candidates);
        bitcounter.truncate_to(bitlength);
        let mask = 1 << (bitlength - i - 1);
        let desired = bitcounter.0[i + (16 - bitlength)].digit() << (bitlength - i - 1);
        candidates.retain(|&n| n & mask == desired);
    }
    assert!(dbg!(&candidates).len() == 1);
    candidates.first().unwrap().to_owned() as u32
}
fn calculate_carbondioxide(input: &str) -> u32 {
    let mut candidates = parse_input_as_binary(input);
    let bitlength = input.lines().next().unwrap().chars().count();
    for i in 0..bitlength {
        let mut bitcounter = BinaryDigitCounters::from_bits(&candidates);
        bitcounter.truncate_to(bitlength);
        let mask = 1 << (bitlength - i - 1);
        let desired = bitcounter.0[i + (16 - bitlength)].not_digit() << (bitlength - i - 1);
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

    const TEST_INPUT: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

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
        fn test_oxygen() {
            let want = 0b10111; // 23
            let oxygen = calculate_oxygen(TEST_INPUT);
            assert_eq!(want, oxygen);
        }

        #[test]
        fn test_carbondioxide() {
            let want = 0b01010; // 10
            let carbondioxide = calculate_carbondioxide(TEST_INPUT);
            assert_eq!(want, carbondioxide);
        }
    }

    #[test]
    fn test_day1_integration() {
        let mut initial: BinaryDigitCounters = Default::default();
        for bits in parse_input_as_binary(TEST_INPUT).iter() {
            initial.push(bits)
        }
        initial.truncate_to(TEST_INPUT.lines().next().unwrap().chars().count());
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
        let mut initial: BinaryDigitCounters = Default::default();
        let bits: u16 = 0b1111111111111111;
        initial.push(&bits);
        assert_eq!(initial.collect_majority(), bits);
        assert_eq!(initial.collect_minority(), 0);

        let mut initial: BinaryDigitCounters = Default::default();
        let bits: u16 = 0b1001001111100100;
        initial.push(&bits);
        assert_eq!(initial.collect_majority(), bits);
        assert_eq!(initial.collect_minority(), !bits);
    }

    #[test]
    fn test_binarydigit_truncate() {
        let mut initial: BinaryDigitCounters = Default::default();
        let bits: u16 = 0b1111111111111111;
        initial.push(&bits);
        initial.truncate_to(12);
        assert_eq!(initial.collect_majority(), 0b111111111111);

        let mut initial: BinaryDigitCounters = Default::default();
        let bits: u16 = 0b0000111111111111;
        initial.push(&bits);
        initial.truncate_to(12);
        assert_eq!(initial.collect_minority(), 0);
    }

    #[test]
    fn test_binarydigit_foldfunc() {
        let mut initial: BinaryDigitCounters = Default::default();
        let bits: u16 = 0b1111111111111011;
        initial.push(&bits);
        let want = BinaryDigitCounters([
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::Zero,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
            BinaryDigitCounter {
                margin: 1,
                majority: BinaryDigit::One,
            },
        ]);
        assert_eq!(initial, want);
    }
}
