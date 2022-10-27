use std::{convert::Infallible, fmt::Display, str::FromStr};

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OpeningSymbol {
    Paren,
    Bracket,
    Brace,
    Angle,
}
impl TryFrom<char> for OpeningSymbol {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::Paren),
            '[' => Ok(Self::Bracket),
            '{' => Ok(Self::Brace),
            '<' => Ok(Self::Angle),
            _ => Err(format!(
                "Can't parse OpeningSymbol from character {:?}",
                value
            )),
        }
    }
}
impl From<OpeningSymbol> for char {
    fn from(symbol: OpeningSymbol) -> Self {
        match symbol {
            OpeningSymbol::Paren => '(',
            OpeningSymbol::Bracket => '[',
            OpeningSymbol::Brace => '{',
            OpeningSymbol::Angle => '<',
        }
    }
}
impl OpeningSymbol {
    fn matching(&self) -> ClosingSymbol {
        match self {
            OpeningSymbol::Paren => ClosingSymbol::Paren,
            OpeningSymbol::Bracket => ClosingSymbol::Bracket,
            OpeningSymbol::Brace => ClosingSymbol::Brace,
            OpeningSymbol::Angle => ClosingSymbol::Angle,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClosingSymbol {
    Paren,
    Bracket,
    Brace,
    Angle,
}
impl TryFrom<char> for ClosingSymbol {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ')' => Ok(Self::Paren),
            ']' => Ok(Self::Bracket),
            '}' => Ok(Self::Brace),
            '>' => Ok(Self::Angle),
            _ => Err(format!(
                "Can't parse ClosingSymbol from character {:?}",
                value
            )),
        }
    }
}
impl From<ClosingSymbol> for char {
    fn from(symbol: ClosingSymbol) -> Self {
        match symbol {
            ClosingSymbol::Paren => ')',
            ClosingSymbol::Bracket => ']',
            ClosingSymbol::Brace => '}',
            ClosingSymbol::Angle => '>',
        }
    }
}
impl Display for ClosingSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch: char = (*self).into();
        write!(f, "{}", ch)
    }
}
impl ClosingSymbol {
    fn matching(&self) -> OpeningSymbol {
        match self {
            ClosingSymbol::Paren => OpeningSymbol::Paren,
            ClosingSymbol::Bracket => OpeningSymbol::Bracket,
            ClosingSymbol::Brace => OpeningSymbol::Brace,
            ClosingSymbol::Angle => OpeningSymbol::Angle,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum SyntaxError {
    CorruptedLine(ClosingSymbol),
    IncompleteLine(Vec<ClosingSymbol>),
}
impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CorruptedLine(symbol) => write!(
                f,
                "Line is corrupted! Found non-matching closing symbol {}",
                symbol
            ),
            Self::IncompleteLine(symbols) => write!(
                f,
                "Line is incomplete. Line would be completed with {}",
                symbols
                    .iter()
                    .map(|symbol| symbol.to_string())
                    .into_iter()
                    .collect::<Vec<String>>()
                    .iter()
                    .as_slice()
                    .join(", ")
            ),
        }
    }
}
impl std::error::Error for SyntaxError {}
impl SyntaxError {
    fn score(&self) -> i64 {
        match self {
            SyntaxError::CorruptedLine(symbol) => match symbol {
                ClosingSymbol::Paren => 3,
                ClosingSymbol::Bracket => 57,
                ClosingSymbol::Brace => 1197,
                ClosingSymbol::Angle => 25137,
            },
            SyntaxError::IncompleteLine(symbols) => symbols.iter().fold(0, |acc, symbol| {
                acc * 5
                    + match symbol {
                        ClosingSymbol::Paren => 1,
                        ClosingSymbol::Bracket => 2,
                        ClosingSymbol::Brace => 3,
                        ClosingSymbol::Angle => 4,
                    }
            }),
        }
    }
}

#[derive(Clone)]
struct Input(String);
impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
impl Input {
    fn iter<'a>(&'a self) -> std::str::Lines {
        self.0.lines()
    }
}

fn validate_line(line: &str) -> Result<String, SyntaxError> {
    let mut stack: Vec<OpeningSymbol> = Vec::new();
    for ch in line.chars() {
        if let Ok(opening_symbol) = OpeningSymbol::try_from(ch) {
            // If the symbol is an opening symbol then push it onto the stack
            stack.push(opening_symbol);
        } else {
            // Otherwise, it must be a closing symbol (or else we panic!)
            // and we should match it against something in the stack already
            let closing_symbol = ClosingSymbol::try_from(ch)
                .expect("Character was neither an opening nor a closing symbol");
            let matching_symbol = closing_symbol.matching();
            loop {
                if let Some(opening_symbol) = stack.pop() {
                    if opening_symbol == matching_symbol {
                        // There's our match. We've popped it off the stack already.
                        break;
                    } else {
                        // If it doesn't match here, this is a CorruptedLine
                        return Err(SyntaxError::CorruptedLine(closing_symbol));
                    }
                } else {
                    // The inner stack is empty, so our closing symbol doesn't
                    // match anything. That's a CorruptedLine
                    return Err(SyntaxError::CorruptedLine(closing_symbol));
                }
            }
        }
    }

    // By the time we get here, stack should be empty. If not it's an IncompleteLine
    if stack.is_empty() {
        Ok(line.into())
    } else {
        Err(SyntaxError::IncompleteLine(
            stack
                .into_iter()
                .rev()
                .map(|opening_symbol| opening_symbol.matching())
                .collect(),
        ))
    }
}

fn solve_part1(input: Input) -> u64 {
    input
        .iter()
        .map(validate_line)
        .map(|validation| -> i64 {
            match validation {
                Ok(_) => 0,
                Err(syntax_error) => match syntax_error {
                    SyntaxError::CorruptedLine(_) => syntax_error.score(),
                    SyntaxError::IncompleteLine(_) => 0,
                },
            }
        })
        .sum::<i64>()
        .try_into()
        .expect("Overflow")
}

fn solve_part2(input: Input) -> u64 {
    let mut incomplete_line_scores: Vec<i64> = input
        .iter()
        .map(validate_line)
        .filter_map(|validation| match validation {
            Ok(_) => None,
            Err(syntax_error) => match syntax_error {
                SyntaxError::IncompleteLine(_) => Some(syntax_error.score()),
                SyntaxError::CorruptedLine(_) => None,
            },
        })
        .collect();

    incomplete_line_scores.sort();
    assert!(incomplete_line_scores.len() % 2 == 1);
    let median_idx = (incomplete_line_scores.len() - 1) / 2;
    incomplete_line_scores[median_idx]
        .try_into()
        .expect("Could not convert i64 to u64")
}

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

    #[test]
    fn solve_part1() {
        let input = INPUT.parse::<Input>().expect("Failed to parse input");
        let result = super::solve_part1(input);
        let expected = 26397;
        assert_eq!(result, expected);
    }

    #[test]
    fn solve_part2() {
        let input = INPUT.parse().expect("Failed to parse input");
        let result = super::solve_part2(input);
        let expected = 288957;
        assert_eq!(result, expected);
    }

    #[test]
    fn validate_corrupt_line() {
        let line = "{([(<{}[<>[]}>{[]{[(<()>";
        let result = validate_line(line);
        let expected = Err(SyntaxError::CorruptedLine(ClosingSymbol::Brace));
        assert_eq!(result, expected);
    }
}
