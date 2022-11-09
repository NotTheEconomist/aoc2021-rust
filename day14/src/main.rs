use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
enum InsertionAction {
    Add(u64),
    Subtract(u64),
}

impl std::ops::Add for InsertionAction {
    type Output = InsertionAction;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            InsertionAction::Add(lhs) => match rhs {
                InsertionAction::Add(rhs) => InsertionAction::Add(lhs + rhs),
                InsertionAction::Subtract(rhs) if lhs >= rhs => InsertionAction::Add(lhs - rhs),
                InsertionAction::Subtract(rhs) if lhs < rhs => InsertionAction::Subtract(rhs - lhs),
                InsertionAction::Subtract(_) => unreachable!(),
            },
            InsertionAction::Subtract(lhs) => match rhs {
                InsertionAction::Add(rhs) if lhs > rhs => InsertionAction::Subtract(lhs - rhs),
                InsertionAction::Add(rhs) if lhs <= rhs => InsertionAction::Add(rhs - lhs),
                InsertionAction::Subtract(rhs) => InsertionAction::Subtract(rhs + lhs),
                InsertionAction::Add(_) => unreachable!(),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PolymerPairCounter {
    doubles: HashMap<(char, char), u64>,
    singles: HashMap<char, u64>,
    insertion_table: HashMap<(char, char), char>,
}
impl From<Input> for PolymerPairCounter {
    fn from(input: Input) -> Self {
        let mut doubles = HashMap::new();
        let mut singles = HashMap::new();
        let insertion_table = input.insertion_table;

        input.polymer_template.chars().for_each(|ch| {
            singles.entry(ch).and_modify(|e| *e += 1).or_insert(1);
        });

        let (cur, mut next) = (
            input.polymer_template.chars(),
            input.polymer_template.chars(),
        );
        next.next(); // advance the second iterator by one
        for pair in cur.zip(next) {
            doubles.entry(pair).and_modify(|e| *e += 1).or_insert(1);
        }

        Self {
            doubles,
            singles,
            insertion_table,
        }
    }
}
impl PolymerPairCounter {
    fn char_counts(self) -> PolymerCounter {
        PolymerCounter(self.singles)
    }
    fn perform_insertions(&mut self) {
        let mut double_insertion_actions: HashMap<(char, char), Vec<InsertionAction>> =
            HashMap::new();
        let mut single_insertion_actions: HashMap<char, Vec<InsertionAction>> = HashMap::new();
        for (&(a, b), &insertion_char) in self.insertion_table.iter() {
            if let Some(count) = self.doubles.get(&(a, b)) {
                // Add the to-be-inserted character to the singles map
                single_insertion_actions
                    .entry(insertion_char)
                    .and_modify(|e| e.push(InsertionAction::Add(*count)))
                    .or_insert_with(|| vec![InsertionAction::Add(*count)]);
                // Add the to-be-inserted character pairs to the doubles map
                for tup in [(a, insertion_char), (insertion_char, b)].into_iter() {
                    double_insertion_actions
                        .entry(tup)
                        .and_modify(|e| e.push(InsertionAction::Add(*count)))
                        .or_insert_with(|| vec![InsertionAction::Add(*count)]);
                }
                // Remove the old pairs from the doubles map
                double_insertion_actions
                    .entry((a, b))
                    .and_modify(|e| e.push(InsertionAction::Subtract(*count)))
                    .or_insert_with(|| vec![InsertionAction::Subtract(*count)]);
            }
        }

        // take the insert actions
        double_insertion_actions
            .into_iter()
            .map(|(key, actions)| -> ((char, char), InsertionAction) {
                (key, actions.into_iter().reduce(std::ops::Add::add).unwrap())
            })
            .for_each(|(key, action)| {
                self.doubles
                    .entry(key)
                    .and_modify(|e| match action {
                        InsertionAction::Add(value) => *e += value,
                        InsertionAction::Subtract(value) => *e -= value,
                    })
                    .or_insert_with_key(|_| match action {
                        InsertionAction::Add(value) => value,
                        InsertionAction::Subtract(value) => {
                            panic!("Can't remove {} from empty key {}{}", value, key.0, key.1);
                        }
                    });
            });
        single_insertion_actions
            .into_iter()
            .map(|(key, actions)| (key, actions.into_iter().reduce(std::ops::Add::add).unwrap()))
            .for_each(|(key, action)| {
                self.singles
                    .entry(key)
                    .and_modify(|e| match action {
                        InsertionAction::Add(value) => *e += value,
                        InsertionAction::Subtract(value) => *e -= value,
                    })
                    .or_insert_with_key(|_| match action {
                        InsertionAction::Add(value) => value,
                        InsertionAction::Subtract(value) => {
                            panic!("Can't remove {} from empty key {}", value, key);
                        }
                    });
            });

        self.doubles.retain(|_, &mut value| value > 0);
    }
}

struct PolymerCounter(HashMap<char, u64>);
impl PolymerCounter {
    fn most_common_count(&self) -> u64 {
        *self
            .0
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(_, count)| count)
            .unwrap()
    }
    fn least_common_count(&self) -> u64 {
        *self
            .0
            .iter()
            .min_by(|a, b| a.1.cmp(b.1))
            .map(|(_, count)| count)
            .unwrap()
    }
}

#[derive(Clone, Debug)]
struct Input {
    polymer_template: String,
    insertion_table: HashMap<(char, char), char>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let polymer_template = lines
            .next()
            .expect("polymer_template must be found as first line of input")
            .to_string();
        lines.next().unwrap(); // One blank line follows the template string
        let mut pair_insertion_table = HashMap::new();
        for line in s.lines() {
            if let Some((from, to)) = line.split_once(" -> ") {
                let mut chars = from.chars();
                let a = chars.next().unwrap();
                let b = chars.next().unwrap();

                let mut chars = to.chars();
                let insertion_character = chars.next().unwrap();
                pair_insertion_table.insert((a, b), insertion_character);
            }
        }
        Ok(Self {
            polymer_template,
            insertion_table: pair_insertion_table,
        })
    }
}

fn solve_part1(input: Input) -> u64 {
    let mut polymer_counter: PolymerPairCounter = input.into();
    for _ in 0..10 {
        polymer_counter.perform_insertions();
    }
    let counter = polymer_counter.char_counts();
    counter.most_common_count() - counter.least_common_count()
}

fn solve_part2(input: Input) -> u64 {
    let mut polymer_counter: PolymerPairCounter = input.into();
    for _ in 0..40 {
        polymer_counter.perform_insertions();
    }
    let counter = polymer_counter.char_counts();
    counter.most_common_count() - counter.least_common_count()
}

fn main() {
    let input = INPUT.parse::<Input>().expect("Input must parse");
    let part1 = solve_part1(input.clone());
    println!("part1: {part1}");
    let part2 = solve_part2(input);
    println!("part2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    #[allow(non_snake_case)]
    fn perform_insertions_PolymerPairCounter() {
        let mut polymer_pair_counter = PolymerPairCounter {
            doubles: HashMap::from([(('N', 'N'), 1), (('N', 'C'), 1), (('C', 'B'), 1)]),
            singles: HashMap::from([('N', 2), ('C', 1), ('B', 1)]),
            insertion_table: HashMap::from([
                (('C', 'B'), 'H'),
                (('N', 'N'), 'C'),
                (('N', 'C'), 'B'),
            ]),
        };
        polymer_pair_counter.perform_insertions();

        let expected = PolymerPairCounter {
            doubles: HashMap::from([
                (('N', 'C'), 1),
                (('C', 'N'), 1),
                (('N', 'B'), 1),
                (('B', 'C'), 1),
                (('C', 'H'), 1),
                (('H', 'B'), 1),
            ]),
            singles: HashMap::from([('N', 2), ('C', 2), ('B', 2), ('H', 1)]),
            insertion_table: HashMap::from([
                (('C', 'B'), 'H'),
                (('N', 'N'), 'C'),
                (('N', 'C'), 'B'),
            ]),
        };

        assert_eq!(polymer_pair_counter, expected);
    }

    #[test]
    fn solve_part1() {
        let input = INPUT.parse::<Input>().expect("Input must parse");
        let result = super::solve_part1(input);

        let expected = 1588;
        assert_eq!(result, expected);
    }
    #[test]
    fn solve_part2() {
        let input = INPUT.parse::<Input>().expect("Input must parse");
        let result = super::solve_part2(input);

        let expected = 2188189693529;
        assert_eq!(result, expected);
    }
}
