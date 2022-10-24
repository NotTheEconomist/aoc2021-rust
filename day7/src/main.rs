#[derive(Clone, Debug)]
struct Input(Vec<i64>);
impl Input {
    fn parse(input: &'static str) -> Result<Self, String> {
        let mut vec = Vec::new();
        for n in input.trim_end().split(',') {
            vec.push(n.parse().map_err(|_| "can't parse value")?)
        }
        Ok(Self(vec))
    }
}

enum CalculationMethod {
    FlatCost,
    IncreasingCost,
}

/// Legacy
fn total_fuel_cost(positions: &Vec<i64>, target_position: i64) -> u64 {
    total_fuel_cost_by_calculation(positions, target_position, CalculationMethod::FlatCost)
}

fn total_fuel_cost_by_calculation(
    positions: &Vec<i64>,
    target_position: i64,
    calculation: CalculationMethod,
) -> u64 {
    positions
        .iter()
        .map(|pos| match calculation {
            CalculationMethod::FlatCost => pos.abs_diff(target_position),
            CalculationMethod::IncreasingCost => (1..=pos.abs_diff(target_position)).sum(),
        })
        .sum()
}

fn solve_part1(input: Input) -> u64 {
    let positions = input.0;
    let (min, max) = (
        *positions.iter().min().unwrap(),
        *positions.iter().max().unwrap(),
    );
    (min..=max)
        .map(|target_position| total_fuel_cost(&positions, target_position))
        .min()
        .unwrap()
}

fn solve_part2(input: Input) -> u64 {
    let positions = input.0;
    let (min, max) = (
        *positions.iter().min().unwrap(),
        *positions.iter().max().unwrap(),
    );
    (min..=max)
        .map(|target_position| {
            total_fuel_cost_by_calculation(
                &positions,
                target_position,
                CalculationMethod::IncreasingCost,
            )
        })
        .min()
        .unwrap()
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = Input::parse(INPUT).expect("failed to parse input");
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
    fn test_solve_part1() {
        let input = Input::parse(INPUT).expect("failed to parse input");
        let got = solve_part1(input);
        let expected = 37;
        assert_eq!(got, expected)
    }

    #[test]
    fn test_solve_part2() {
        let input = Input::parse(INPUT).expect("failed to parse input");
        let got = solve_part2(input);
        let expected = 168;
        assert_eq!(got, expected)
    }
}
