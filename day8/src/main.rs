use std::str::FromStr;

struct SevenSegmentDisplayOutput([Digit; 4]);

impl From<SevenSegmentDisplayOutput> for u64 {
    fn from(ssdo: SevenSegmentDisplayOutput) -> Self {
        let [a, b, c, d] = ssdo.0;
        let a: u64 = a.into();
        let b: u64 = b.into();
        let c: u64 = c.into();
        let d: u64 = d.into();

        a * 1000 + b * 100 + c * 10 + d
    }
}

#[derive(Debug)]
struct SevenSegmentDisplay {
    outputs: [WiringSegment; 4],
    mapping: [Digit; 10],
}

impl SevenSegmentDisplay {
    fn new(entry: Entry) -> Self {
        let mut mapping = [Digit::Zero(WiringSegment(0)); 10];
        let segments = entry.segments;

        // fill in the obvious entries first
        for segment in segments.iter() {
            if let Some((idx, digit)) = match segment.count_segments() {
                2 => Some((1, Digit::One(*segment))),
                3 => Some((7, Digit::Seven(*segment))),
                4 => Some((4, Digit::Four(*segment))),
                7 => Some((8, Digit::Eight(*segment))),
                _ => None,
            } {
                mapping[idx] = digit;
            }
        }

        // To find the rest of the digits, we have to start inferring Some
        // WiringSegment locations. For instance, 4 (known from above) shares
        // ONLY the middle segment with 2 & 3 & 5. Since we can isolate
        // 2, 3, and 5 by selecting for .count_segments() == 5, we can
        // deterministically find the mask for the middle segment.
        let mut middle_segment_mask: u8 = mapping[4].get_wiring_segment().0;
        let two_three_five_segments = entry.segments.iter().filter_map(|segment| -> Option<u8> {
            if segment.count_segments() == 5 {
                Some(segment.0)
            } else {
                None
            }
        });

        for segment in two_three_five_segments.clone() {
            middle_segment_mask &= segment;
        }

        // We can find Digit::Zero by looking for the segment with 6 sections
        // that does not contain the middle
        let zero_segment = segments
            .iter()
            .filter(|segment| segment.count_segments() == 6)
            .find(|segment| segment.0 & middle_segment_mask == 0)
            .copied()
            .unwrap();
        mapping[0] = Digit::Zero(zero_segment);

        // Digit::Six can be found by XOR'ing with 8 and asserting that & 4 is 0
        // Digit::Nine can be found in a similar way, but asserting that & 4 is >0
        let six_segment = segments
            .iter()
            .filter(|&segment| segment.count_segments() == 6)
            .filter(|&segment| segment.0 & zero_segment.0 != zero_segment.0)
            .find(|&segment| {
                (segment.0 ^ mapping[8].get_wiring_segment().0) & mapping[4].get_wiring_segment().0
                    > 0
            })
            .copied()
            .unwrap();
        mapping[6] = Digit::Six(six_segment);
        let nine_segment = segments
            .iter()
            .filter(|&segment| segment.count_segments() == 6)
            .filter(|&segment| segment.0 & zero_segment.0 != zero_segment.0)
            .find(|&segment| {
                (segment.0 ^ mapping[8].get_wiring_segment().0) & mapping[4].get_wiring_segment().0
                    == 0
            })
            .copied()
            .unwrap();
        mapping[9] = Digit::Nine(nine_segment);

        // Digit::Three can be differentiating between two and five
        let three_segment = two_three_five_segments
            .clone()
            .find(|segment| {
                segment & mapping[7].get_wiring_segment().0 == mapping[7].get_wiring_segment().0
            })
            .unwrap();
        mapping[3] = Digit::Three(WiringSegment(three_segment));

        // Digit::Five remains the same when & Six, two does not.
        let two_five_segment =
            two_three_five_segments.filter(|&segment| segment & three_segment != segment);

        for segment in two_five_segment {
            let (idx, digit) = match segment & mapping[6].get_wiring_segment().0 == segment {
                true => (5, Digit::Five(WiringSegment(segment))),
                false => (2, Digit::Two(WiringSegment(segment))),
            };
            mapping[idx] = digit;
        }

        Self {
            outputs: entry.outputs,
            mapping,
        }
    }
    fn digits(&self) -> [Digit; 4] {
        self.outputs
            .iter()
            .map(|ws| -> Digit {
                *self
                    .mapping
                    .iter()
                    .find(|&d| ws == d.get_wiring_segment())
                    .unwrap()
            })
            .collect::<Vec<Digit>>()
            .try_into()
            .unwrap()
    }
}

/// A digit on a seven-segment-display
#[derive(Hash, Copy, Eq, PartialEq, Clone, Debug)]
enum Digit {
    Zero(WiringSegment),
    One(WiringSegment),
    Two(WiringSegment),
    Three(WiringSegment),
    Four(WiringSegment),
    Five(WiringSegment),
    Six(WiringSegment),
    Seven(WiringSegment),
    Eight(WiringSegment),
    Nine(WiringSegment),
}

impl Digit {
    fn get_wiring_segment(&self) -> &WiringSegment {
        match self {
            Digit::Zero(ws) => ws,
            Digit::One(ws) => ws,
            Digit::Two(ws) => ws,
            Digit::Three(ws) => ws,
            Digit::Four(ws) => ws,
            Digit::Five(ws) => ws,
            Digit::Six(ws) => ws,
            Digit::Seven(ws) => ws,
            Digit::Eight(ws) => ws,
            Digit::Nine(ws) => ws,
        }
    }
}

impl From<Digit> for u64 {
    fn from(d: Digit) -> Self {
        match d {
            Digit::Zero(_) => 0,
            Digit::One(_) => 1,
            Digit::Two(_) => 2,
            Digit::Three(_) => 3,
            Digit::Four(_) => 4,
            Digit::Five(_) => 5,
            Digit::Six(_) => 6,
            Digit::Seven(_) => 7,
            Digit::Eight(_) => 8,
            Digit::Nine(_) => 9,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct WiringSegment(u8);

impl WiringSegment {
    fn count_segments(&self) -> usize {
        let mut inner = self.0;
        let mut count = 0;
        while inner > 0 {
            // if the least-significant bit is 1, add one. Else zero
            count += inner & 0b1;

            // right-shift the least-significant bit off
            inner >>= 1;

            // continue until the value has no more bits
        }
        count as usize
    }
}

impl FromStr for WiringSegment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments: u8 = 0;
        for c in s.chars() {
            match c {
                'a' => segments |= 1 << 0,
                'b' => segments |= 1 << 1,
                'c' => segments |= 1 << 2,
                'd' => segments |= 1 << 3,
                'e' => segments |= 1 << 4,
                'f' => segments |= 1 << 5,
                'g' => segments |= 1 << 6,
                _ => return Err("Bad input string".to_string()),
            }
        }
        Ok(Self(segments))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Entry {
    segments: [WiringSegment; 10],
    outputs: [WiringSegment; 4],
}
impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (segments, outputs) = s.trim_end().split_once(" | ").unwrap();
        let segments: [WiringSegment; 10] = segments
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<WiringSegment>().expect("Invalid WiringSegment"))
            .collect::<Vec<WiringSegment>>()
            .try_into()
            .unwrap();
        let outputs: [WiringSegment; 4] = outputs
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<WiringSegment>().expect("Invalid WiringSegment"))
            .collect::<Vec<WiringSegment>>()
            .try_into()
            .unwrap();

        Ok(Self { segments, outputs })
    }
}

#[derive(Clone, Debug)]
struct Input(Vec<Entry>);
impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.parse::<Entry>().expect("Invalid entry"))
                .collect(),
        ))
    }
}

impl IntoIterator for Input {
    type Item = Entry;

    type IntoIter = std::vec::IntoIter<Entry>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Input {
    fn iter(&self) -> std::slice::Iter<Entry> {
        self.0.iter()
    }
}

fn solve_part2(input: Input) -> u64 {
    input
        .into_iter()
        .map(SevenSegmentDisplay::new)
        .map(|ssd| -> u64 { SevenSegmentDisplayOutput(ssd.digits()).into() })
        .sum()
}

fn solve_part1(input: Input) -> u64 {
    let unique_segment_counts = [2, 3, 4, 7];
    input
        .iter()
        .flat_map(|entry| {
            entry
                .outputs
                .iter()
                .filter(|segment| unique_segment_counts.contains(&segment.count_segments()))
        })
        .count()
        .try_into()
        .unwrap()
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = INPUT.parse::<Input>().expect("Failed to parse input");
    let part1 = solve_part1(input.clone());
    println!("part1: {}", part1);
    let part2 = solve_part2(input);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    macro_rules! ws {
        ($w:literal) => {
            $w.parse::<WiringSegment>()
                .expect("Failed to parse wiring segments")
        };
    }

    #[test]
    fn parse_mappings() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let input = input.parse::<Input>().expect("Failed to parse input");
        let entry = input.0.into_iter().next().unwrap();

        let ssd = SevenSegmentDisplay::new(entry);
        let expected = [
            Digit::Zero(ws!("cagedb")),
            Digit::One(ws!("ab")),
            Digit::Two(ws!("gcdfa")),
            Digit::Three(ws!("fbcad")),
            Digit::Four(ws!("eafb")),
            Digit::Five(ws!("cdfbe")),
            Digit::Six(ws!("cdfgeb")),
            Digit::Seven(ws!("dab")),
            Digit::Eight(ws!("acedgfb")),
            Digit::Nine(ws!("cefabd")),
        ];

        assert_eq!(ssd.mapping, expected);
    }

    #[test]
    fn solve_part1() {
        let input = INPUT.parse::<Input>().expect("Failed to parse input");
        let result = super::solve_part1(input);
        let expect = 26;

        assert_eq!(result, expect);
    }

    #[test]
    fn solve_part2() {
        let input = INPUT.parse::<Input>().expect("Failed to parse input");
        let result = super::solve_part2(input);
        let expect = 61229;

        assert_eq!(result, expect);
    }

    #[test]
    fn parse_entry() {
        let input: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let entry = input.parse::<Entry>().expect("Failed to parse entry");
        let expected = Entry {
            segments: [
                ws!("be"),
                ws!("cfbegad"),
                ws!("cbdgef"),
                ws!("fgaecd"),
                ws!("cgeb"),
                ws!("fdcge"),
                ws!("agebfd"),
                ws!("fecdb"),
                ws!("fabcd"),
                ws!("edb"),
            ],
            outputs: [ws!("fdgacbe"), ws!("cefdb"), ws!("cefbgd"), ws!("gcbe")],
        };

        assert_eq!(entry, expected);
    }

    /// If two diagrams have the same characters,
    /// they should be identical. Order is irrelevant
    #[test]
    fn different_wirings_are_identical() {
        let (a, b) = ("abcd", "dcab");
        assert_eq!(
            a.parse::<WiringSegment>().unwrap(),
            b.parse::<WiringSegment>().unwrap()
        );
    }

    // Regression tests
    #[test]
    fn gadfec_equals_fgdeca() {
        let (a, b) = ("gadfec", "fgdeca");
        assert_eq!(
            a.parse::<WiringSegment>().unwrap(),
            b.parse::<WiringSegment>().unwrap(),
        );
    }

    #[test]
    fn bad_inferrence_of_six_in_some_cases() {
        let input =
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"
                .parse::<Input>()
                .expect("Failed to parse input");
        let entry = input.0.into_iter().next().unwrap();
        let ssd = SevenSegmentDisplay::new(entry);
        let expected = [
            Digit::Zero(ws!("bcdefg")),
            Digit::One(ws!("bc")),
            Digit::Two(ws!("abdge")),
            Digit::Three(ws!("abcde")),
            Digit::Four(ws!("abcf")),
            Digit::Five(ws!("acdef")),
            Digit::Six(ws!("acdefg")),
            Digit::Seven(ws!("bcd")),
            Digit::Eight(ws!("abcdefg")),
            Digit::Nine(ws!("abcdef")),
        ];

        assert_eq!(ssd.mapping, expected);
    }
}
