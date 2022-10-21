use std::{ops::Add, str::FromStr};

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let command = parts
            .next()
            .ok_or(format!("{} does not have a command part", s))?;
        let value = parts
            .next()
            .ok_or(format!("{} does not have a value part", s))
            .map(|v| v.parse().unwrap())?;
        match command {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err(format!("{} has invalid command part {}", s, command)),
        }
    }
}

#[derive(Default, PartialEq, Eq, Debug)]
struct Position {
    horizontal: u32,
    depth: u32,
    aim: Heading,
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    #[allow(dead_code)]
    Up,
    Down,
}

#[derive(PartialEq, Eq, Debug)]
struct Heading {
    direction: Direction,
    magnitude: u32,
}

impl Default for Heading {
    fn default() -> Self {
        Self {
            direction: Direction::Down,
            magnitude: Default::default(),
        }
    }
}

impl Add<Command> for Heading {
    type Output = Self;

    fn add(self, rhs: Command) -> Self::Output {
        match (rhs, &self.direction) {
            (Command::Down(val), Direction::Down) => Self {
                magnitude: self.magnitude + val,
                ..self
            },
            (Command::Down(val), Direction::Up) => {
                if val > self.magnitude {
                    Self {
                        magnitude: val - self.magnitude,
                        direction: Direction::Up,
                    }
                } else {
                    Self {
                        magnitude: self.magnitude - val,
                        ..self
                    }
                }
            }
            (Command::Up(val), Direction::Up) => Self {
                magnitude: self.magnitude + val,
                ..self
            },
            (Command::Up(val), Direction::Down) => {
                if val > self.magnitude {
                    Self {
                        magnitude: val - self.magnitude,
                        direction: Direction::Down,
                    }
                } else {
                    Self {
                        magnitude: self.magnitude - val,
                        ..self
                    }
                }
            }
            (Command::Forward(_), _) => self
        }
    }
}

impl Position {
    fn act(self, command: Command) -> Self {
        match command {
            Command::Forward(x) => Self {
                horizontal: self.horizontal + x,
                ..self
            },
            Command::Down(y) => Self {
                depth: self.depth + y,
                ..self
            },
            Command::Up(y) => Self {
                depth: self.depth - y,
                ..self
            },
        }
    }
    fn act_v2(self, command: Command) -> Self {
        match command {
            Command::Forward(x) => Self {
                horizontal: self.horizontal + x,
                depth: match self.aim.direction {
                    Direction::Down => self.depth + x * self.aim.magnitude,
                    Direction::Up => self.depth - x * self.aim.magnitude,
                },
                ..self
            },
            Command::Up(_) => Self {
                aim: self.aim + command,
                ..self
            },
            Command::Down(_) => Self {
                aim: self.aim + command,
                ..self
            },
        }
    }
    fn get_value(&self) -> u32 {
        self.horizontal * self.depth
    }
}

const INPUT: &str = include_str!("input.txt");

type Calculator = &'static dyn Fn(Position, Command) -> Position;

fn parse_and_run_commands_with(input: &str, calculator: Calculator) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .fold(Position::default(), calculator)
        .get_value()
}

fn main() {
    let part1 = parse_and_run_commands_with(INPUT, &Position::act);
    println!("part1: {}", part1);

    let part2 = parse_and_run_commands_with(INPUT, &Position::act_v2);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_calculate() {
        let want = Position {
            horizontal: 3,
            depth: 3,
            ..Default::default()
        };
        let got = [
            Command::Forward(3),
            Command::Down(2),
            Command::Down(2),
            Command::Up(1),
        ]
        .into_iter()
        .fold(Default::default(), Position::act);
        assert_eq!(got, want);
    }

    #[test]
    fn test_position_get_value() {
        assert_eq!(
            Position {
                horizontal: 4,
                depth: 3,
                ..Default::default()
            }
            .get_value(),
            12
        );
    }

    #[test]
    fn test_day1() {
        let input = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(parse_and_run_commands_with(input, &Position::act), 150)
    }
}
