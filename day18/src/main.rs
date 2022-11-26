use day18::SnailFish;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Input {
    numbers: Vec<SnailFish>,
}

impl From<Vec<SnailFish>> for Input {
    fn from(numbers: Vec<SnailFish>) -> Self {
        Self { numbers }
    }
}

fn solve_part1(input: Input) -> Option<u64> {
    Some(
        input
            .numbers
            .into_iter()
            .reduce(|acc, next| acc + next)?
            .magnitude(),
    )
}

fn solve_part2(input: Input) -> Option<u64> {
    let cloned_numbers = input.clone().numbers;
    input
        .numbers
        .into_iter()
        .cartesian_product(cloned_numbers)
        .filter(|(a, b)| a != b)
        .map(|(a, b)| a + b)
        .map(|fish| fish.magnitude())
        .max()
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input: Input = INPUT
        .lines()
        .map(|line| line.parse().expect("Input must parse"))
        .collect::<Vec<_>>()
        .into();
    let part1 = solve_part1(input.clone()).expect("part1 must have a solution");
    println!("part1: {part1}");
    let part2 = solve_part2(input).expect("part2 must have a solution");
    println!("part2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn solve_part1() {
        let input: Input = INPUT
            .lines()
            .map(|line| {
                line.parse::<SnailFish>()
                    .expect("Input lines must all parse")
            })
            .collect::<Vec<_>>()
            .into();
        let result = super::solve_part1(input);
        assert_eq!(result, Some(4140))
    }
    #[test]
    fn solve_part2() {
        let input: Input = INPUT
            .lines()
            .map(|line| {
                line.parse::<SnailFish>()
                    .expect("Input lines must all parse")
            })
            .collect::<Vec<_>>()
            .into();
        let result = super::solve_part2(input);
        assert_eq!(result, Some(3993))
    }

    #[test]
    fn test_sum() {
        let input_fish = INPUT
            .lines()
            .map(|line| {
                line.parse::<SnailFish>()
                    .expect("Input lines must all parse")
            })
            .reduce(|acc, next| acc + next)
            .expect("input is nonempty");

        let expected: SnailFish = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
            .parse()
            .expect("expected must parse");
        assert_eq!(input_fish, expected);
    }
}
