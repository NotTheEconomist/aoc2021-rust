use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug, Hash)]
struct Point {
    x: u64,
    y: u64,
}

impl From<(u64, u64)> for Point {
    fn from((x, y): (u64, u64)) -> Self {
        Point { x, y }
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or(String::from("Can't split line on comma"))?;
        Ok(Point {
            x: x.parse()
                .map_err(|_| String::from("x does not parse to u64"))?,
            y: y.parse()
                .map_err(|_| String::from("y does not parse to u64"))?,
        })
    }
}

#[derive(Debug, Clone)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (orientation, value) = s.split_once('=').ok_or(String::from(
            "Can't split fold between orientation and value",
        ))?;

        let value = value
            .parse()
            .map_err(|_| String::from("Can't parse value to usize"))?;

        match orientation {
            "fold along y" => Ok(Self::Horizontal(value)),
            "fold along x" => Ok(Self::Vertical(value)),
            _ => Err(String::from("orientation is malformed")),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Grid(HashSet<Point>);
impl Grid {
    fn fold(self, fold: Fold) -> Self {
        let fold_map: Box<dyn Fn(Point) -> Point> = match fold {
            Fold::Horizontal(value) => Box::new(move |point: Point| -> Point {
                if point.y > value as u64 {
                    Point {
                        y: (value as u64 - (point.y - value as u64)),
                        ..point
                    }
                } else {
                    point
                }
            }),
            Fold::Vertical(value) => Box::new(move |point: Point| -> Point {
                if point.x > value as u64 {
                    Point {
                        x: (value as u64 - (point.x - value as u64)),
                        ..point
                    }
                } else {
                    point
                }
            }),
        };
        Self(self.0.into_iter().map(fold_map).collect::<HashSet<Point>>())
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self
            .0
            .iter()
            .max_by(|&a, &b| a.x.cmp(&b.x))
            .map(|point| point.x + 1)
            .unwrap();
        // The height of the grid is the y of the furthest-down point plus 1
        let height = self
            .0
            .iter()
            .max_by(|&a, &b| a.y.cmp(&b.y))
            .map(|point| point.y + 1)
            .unwrap();

        let mut lines: Vec<String> = Vec::new();
        for y in 0..height {
            let mut line = String::new();
            for x in 0..width {
                let ch = if self.0.contains(&Point { x, y }) {
                    '#'
                } else {
                    '.'
                };
                line.push(ch);
            }
            lines.push(line);
        }

        write!(f, "{}", lines.join("\n"))
    }
}

impl From<Input> for Grid {
    fn from(input: Input) -> Self {
        Grid(input.points)
    }
}

#[derive(Debug, Clone)]
struct Input {
    points: HashSet<Point>,
    folds: Vec<Fold>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = HashSet::new();
        let mut folds = Vec::new();

        for line in s.lines() {
            if let Ok(point) = line.parse::<Point>() {
                points.insert(point);
            } else if let Ok(fold) = line.parse::<Fold>() {
                folds.push(fold)
            } else if line.is_empty() {
                continue;
            } else {
                return Err(String::from("Failed to parse line"));
            }
        }

        Ok(Self { points, folds })
    }
}

fn solve_part1(input: Input) -> u64 {
    let mut grid = Grid(input.points);
    if let Some(fold) = input.folds.into_iter().next() {
        grid = grid.fold(fold)
    }

    return grid.0.len() as u64;
}

fn solve_part2(input: Input) -> Grid {
    let mut grid = Grid(input.points);
    grid = input
        .folds
        .into_iter()
        .fold(grid, |grid, fold| grid.fold(fold));
    grid
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = INPUT.parse::<Input>().expect("Input must parse");
    let part1 = solve_part1(input.clone());
    println!("part1: {part1}");
    let part2 = solve_part2(input);
    println!("part2:\n{part2}");
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_fold_once_simple() {
        /*
           | * * |
           |   * |
           | --- |
           | *   |
           |     |
        */
        let grid = Grid(HashSet::from([
            (0, 0).into(),
            (1, 0).into(),
            (1, 1).into(),
            (0, 3).into(),
        ]));
        /*
           | * * |
           | * * |
        */
        let expected = Grid(HashSet::from([
            (0, 0).into(),
            (1, 0).into(),
            (1, 1).into(),
            (0, 1).into(),
        ]));
        assert_eq!(grid.fold(Fold::Horizontal(2)), expected);
    }

    #[test]
    fn test_fold_once_squish() {
        /*
           | * * |
           | * * |
           | --- |
           | *   |
           |     |
        */
        let grid = Grid(HashSet::from([
            (0, 1).into(),
            (0, 0).into(),
            (1, 0).into(),
            (1, 1).into(),
            (0, 3).into(),
        ]));
        /*
           | * * |
           | * * |
        */
        let expected = Grid(HashSet::from([
            (0, 0).into(),
            (1, 0).into(),
            (1, 1).into(),
            (0, 1).into(),
        ]));
        assert_eq!(grid.fold(Fold::Horizontal(2)), expected);
    }

    #[test]
    fn solve_part1() {
        let input = INPUT.parse::<Input>().expect("Input must parse");
        let result = super::solve_part1(input);

        let expected = 17;
        assert_eq!(result, expected)
    }

    #[test]
    fn solve_part2() {
        let input = INPUT.parse::<Input>().expect("Input must parse");
        let result = super::solve_part2(input);

        let expected = Grid(HashSet::from([
            (0, 0).into(),
            (1, 0).into(),
            (2, 0).into(),
            (3, 0).into(),
            (4, 0).into(),
            (0, 1).into(),
            (4, 1).into(),
            (0, 2).into(),
            (4, 2).into(),
            (0, 3).into(),
            (4, 3).into(),
            (0, 4).into(),
            (1, 4).into(),
            (2, 4).into(),
            (3, 4).into(),
            (4, 4).into(),
        ]));

        assert_eq!(result, expected);
    }
}
