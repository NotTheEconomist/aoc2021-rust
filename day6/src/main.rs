#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct FishState([u64; 9]);
impl FishState {
    fn tick(&mut self) {
        self.0.rotate_left(1);
        self.0[6] += self.0[8];
    }

    fn count(&self) -> u64 {
        self.0.iter().sum()
    }
}

struct State {
    fishes: FishState,
    tick: i32,
}

impl State {
    fn tick(&mut self) {
        self.fishes.tick();
        self.tick += 1;
    }

    fn new(fishes: Vec<i32>) -> Self {
        let mut acc = [0; 9];
        fishes.iter().for_each(|n| {
            acc[*n as usize] += 1;
        });
        let fishes = FishState(acc);
        Self { fishes, tick: 0 }
    }
}

impl Iterator for State {
    type Item = FishState;

    fn next(&mut self) -> Option<Self::Item> {
        self.tick();
        Some(self.fishes)
    }
}

#[derive(Clone)]
struct Input(Vec<i32>);
impl Input {
    fn parse(input: &'static str) -> Result<Self, String> {
        let input = input.trim().split(',');
        let mut result = Vec::new();
        for n in input {
            let n = match n.parse() {
                Ok(n) => Ok(n),
                Err(_) => Err(format!("failed to parse number {:?} in input", n)),
            }?;
            result.push(n);
        }
        Ok(Self(result))
    }
}

fn solve_part1(input: Input) -> u64 {
    let state = State::new(input.0);
    let ticks = 80;
    match state.into_iter().nth(ticks - 1) {
        Some(newstate) => newstate.count(),
        None => panic!("this error should never happen"),
    }
}

fn solve_part2(input: Input) -> u64 {
    let state = State::new(input.0);
    let ticks = 256;
    let after_ticks = match state.into_iter().nth(ticks - 1) {
        Some(newstate) => newstate,
        None => panic!("this error should never happen"),
    };
    after_ticks.count()
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = Input::parse(INPUT).expect("Failed to parse input");
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
        let input = Input::parse(INPUT).expect("Failed to parse input");
        let part1 = solve_part1(input);
        let expected = 5934u64;
        assert_eq!(part1, expected);
    }
    #[test]
    fn test_solve_part2() {
        let input = Input::parse(INPUT).expect("Failed to parse input");
        let part2 = solve_part2(input);
        let expected = 26984457539u64;
        assert_eq!(part2, expected);
    }

    #[test]
    fn test_tick() {
        let mut state = State::new(Input::parse(INPUT).expect("Failed to parse input").0);
        state.tick();
        assert_eq!(&state.fishes.0, &[1, 1, 2, 1, 0, 0, 0, 0, 0]);
        state.tick();
        assert_eq!(&state.fishes.0, &[1, 2, 1, 0, 0, 0, 1, 0, 1]);
        state.tick();
        assert_eq!(&state.fishes.0, &[2, 1, 0, 0, 0, 1, 1, 1, 1]);
        state.tick();
        assert_eq!(&state.fishes.0, &[1, 0, 0, 0, 1, 1, 3, 1, 2]);
        state.tick();
        assert_eq!(&state.fishes.0, &[0, 0, 0, 1, 1, 3, 2, 2, 1]);
    }
}
