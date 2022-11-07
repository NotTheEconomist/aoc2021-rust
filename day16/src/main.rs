use std::{convert::Infallible, str::FromStr};

use day16::*;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Input(String);
impl ToString for Input {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input(s.to_string()))
    }
}

fn solve_part1(input: Input) -> u64 {
    let packet = Packet::from_hex_str(&input.to_string()).expect("Input must parse");
    vec![&packet]
        .into_iter()
        .chain(packet.traverse_subpackets())
        .map(|packet| packet.version)
        .sum::<u64>()
}

fn solve_part2(input: Input) -> u64 {
    let packet = Packet::from_hex_str(&input.to_string()).expect("Input must parse");
    packet.value()
}

fn main() {
    let input = INPUT.parse::<Input>().expect("Input must parse");
    let part1 = solve_part1(input.clone());
    println!("part1: {part1}");
    let part2 = solve_part2(input);
    println!("part2: {part2}");
}
