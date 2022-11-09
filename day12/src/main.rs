use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

#[derive(PartialEq, Eq, Debug, Clone)]
enum CaveSize {
    Small,
    Large,
    Start,
    End,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Cave {
    size: CaveSize,
    ident: String,
    paths: Vec<String>,
}

type CaveSystem = HashMap<String, Cave>;

impl From<Input> for CaveSystem {
    fn from(input: Input) -> Self {
        input.system
    }
}

// A path tuple of (path, has_backtracked)
type Path = (Vec<String>, bool);

impl Cave {
    fn get_neighbors<'a>(&'a self, system: &'a CaveSystem) -> Vec<&Cave> {
        self.paths
            .iter()
            .flat_map(|name: &String| -> Option<&Cave> { system.get(name) })
            .collect()
    }

    fn traverse_path_part_two<'a>(&'a self, path: Path, system: &'a CaveSystem) -> Vec<Path> {
        let (path, has_backtracked) = path;
        self.get_neighbors(system)
            .into_iter()
            // If the next cave is
            // * visited already in this path
            // * a small cave
            // and
            // * we've already backtracked once
            // or seperately
            // * the start cave
            // then filter this neighbor out of future searches
            .filter_map(|next| -> Option<Path> {
                if (path.contains(&next.ident) && next.size == CaveSize::Small && has_backtracked)
                    || next.size == CaveSize::Start
                {
                    None
                } else {
                    // If we've already backtracked
                    // OR
                    // we're backtracking right now
                    let new_has_backtracked = has_backtracked
                        || next.size == CaveSize::Small
                            && path.iter().any(|previous| previous == &next.ident);

                    let mut newpath = path.clone();
                    newpath.push(next.ident.clone());
                    Some((newpath, new_has_backtracked))
                }
            })
            .collect()
    }

    fn traverse_path<'a>(&'a self, path: Vec<String>, system: &'a CaveSystem) -> Vec<Vec<String>> {
        self.get_neighbors(system)
            .into_iter()
            .filter_map(|next| -> Option<Vec<String>> {
                if (path.contains(&next.ident) && next.size == CaveSize::Small)
                    || next.size == CaveSize::Start
                {
                    None
                } else {
                    let mut newpath = path.clone();
                    newpath.push(next.ident.clone());
                    Some(newpath)
                }
            })
            .collect()
    }
}

impl FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = match s {
            "start" => Some(CaveSize::Start),
            "end" => Some(CaveSize::End),
            s if s.to_uppercase() == s => Some(CaveSize::Large),
            s if s.to_lowercase() == s => Some(CaveSize::Small),
            _ => None,
        }
        .ok_or(format!("Can't parse size from {s}"))?;
        Ok(Self {
            size,
            ident: s.to_string(),
            paths: Vec::new(),
        })
    }
}

#[derive(Clone, Debug)]
struct Input {
    system: CaveSystem,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut system: CaveSystem = HashMap::new();

        for line in s.lines() {
            if let Some((from, to)) = line.split_once('-') {
                let from_cave = system
                    .entry(from.to_string())
                    .or_insert(from.parse::<Cave>()?);
                from_cave.paths.push(to.to_string());
                let to_cave = system.entry(to.to_string()).or_insert(to.parse::<Cave>()?);
                to_cave.paths.push(from.to_string());
            }
        }
        Ok(Self { system })
    }
}

fn solve_part1(input: Input) -> u64 {
    let system: CaveSystem = input.into();
    let mut result = 0;

    // acc begins as a vector of vectors each with one element, the neighbors of start
    let mut acc: Vec<Vec<String>> = system
        .get("start")
        .expect("All cave systems must contain a 'start' node.")
        .paths
        .clone()
        .into_iter()
        .map(|head| vec![head])
        .collect();
    while let Some(path) = acc.pop() {
        let cave = &path[&path.len() - 1];
        let cave = system
            .get(cave)
            .expect("Every cave should appear in the system");
        if cave.size == CaveSize::End {
            // We've found a path to the exit! Result +=1 and continue
            result += 1;
            continue;
        }
        for neighbor_path in cave.traverse_path(path, &system).into_iter() {
            acc.push(neighbor_path);
        }
    }
    result
}

fn solve_part2(input: Input) -> u64 {
    let system: CaveSystem = input.into();
    let mut result = 0;

    // acc begins as a vector of vectors each with one element, the neighbors of start
    let mut acc: Vec<Path> = system
        .get("start")
        .expect("All cave systems must contain a 'start' node.")
        .paths
        .clone()
        .into_iter()
        .map(|head| (vec![head], false))
        .collect();
    while let Some((path, small_cave_to_revisit)) = acc.pop() {
        let cave = &path[&path.len() - 1];
        let cave = system
            .get(cave)
            .expect("Every cave should appear in the system");
        if cave.size == CaveSize::End {
            // We've found a path to the exit! Result +=1 and continue
            result += 1;
            continue;
        }
        for neighbor_path in cave
            .traverse_path_part_two((path, small_cave_to_revisit), &system)
            .into_iter()
        {
            acc.push(neighbor_path);
        }
    }
    result
}

fn main() {
    let input = INPUT.parse::<Input>().expect("Input should parse");
    let part1 = solve_part1(input);
    println!("part1: {part1}");
    let input = INPUT.parse::<Input>().expect("Input should parse");
    let part2 = solve_part2(input);
    println!("part2: {part2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    #[test]
    fn solve_part1() {
        let input = INPUT.parse::<Input>().expect("Input should parse");
        let result = super::solve_part1(input);
        assert_eq!(result, 19);
    }
    #[test]
    fn solve_part2() {
        let input = INPUT.parse::<Input>().expect("Input should parse");
        let result = super::solve_part2(input);
        assert_eq!(result, 103);
    }

    #[test]
    fn test_traverse_simple() {
        // Traverse expects a cave system, so let's start there
        let system: CaveSystem = "\
start-a
a-end"
            .parse::<Input>()
            .unwrap()
            .into();
        let start_node = system.get("start").unwrap();
        let result = start_node.traverse_path(vec!["start".to_string()], &system);
        assert_eq!(result, vec![vec!["start", "a"]])
    }
    #[test]
    fn test_traverse_two_simple() {
        // Traverse expects a cave system, so let's start there
        let system: CaveSystem = "\
start-a
a-end"
            .parse::<Input>()
            .unwrap()
            .into();
        let start_node = system.get("start").unwrap();
        let result = start_node.traverse_path_part_two((vec!["start".to_string()], false), &system);
        assert_eq!(
            result,
            vec![(vec![String::from("start"), String::from("a")], false)]
        )
    }

    #[test]
    fn test_traverse_two_allow_backtrack() {
        // Traverse expects a cave system, so let's start there
        let system: CaveSystem = "\
start-a
a-B
B-end
a-end"
            .parse::<Input>()
            .unwrap()
            .into();
        let start_node = system.get("start").unwrap();
        let result = start_node.traverse_path_part_two(
            (
                vec![String::from("start"), String::from("a"), String::from("B")],
                false,
            ),
            &system,
        );
        assert!(result.contains(&(
            vec![
                String::from("start"),
                String::from("a"),
                String::from("B"),
                String::from("a")
            ],
            true
        )))
    }
}
