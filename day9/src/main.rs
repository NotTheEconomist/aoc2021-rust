use itertools::{iproduct, Itertools};

use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cell<T: Eq> {
    value: T,
    coords: (usize, usize),
}

impl<'a> Cell<u8> {
    fn get_neighbors(&self, grid: &'a Grid<u8>) -> Vec<&'a Self> {
        let mut neighbors = Vec::new();

        // Constructing coordinates this way allows us to avoid doing repeated bounds checking
        let neighbor_coords: Vec<(usize, usize)> = [
            (self.coords.0.checked_sub(1), Some(self.coords.1)),
            (Some(self.coords.0), self.coords.1.checked_sub(1)),
            (self.coords.0.checked_add(1), Some(self.coords.1)),
            (Some(self.coords.0), self.coords.1.checked_add(1)),
        ]
        .into_iter()
        .filter_map(|coords| -> Option<(usize, usize)> {
            if let (Some(x), Some(y)) = coords {
                Some((x, y))
            } else {
                None
            }
        })
        .collect();
        for coords in neighbor_coords {
            if let Some(cell) = grid.get(coords) {
                neighbors.push(cell);
            }
        }

        neighbors
    }
}

impl<T> Display for Cell<T>
where
    T: Eq + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Cell<T>
where
    T: Eq,
    T: Clone + Into<u64>,
{
    fn get_risk_level(&self) -> u64 {
        self.value.clone().into() + 1
    }
}

#[derive(Debug, Clone)]
struct Basin<'a> {
    members: Vec<&'a Cell<u8>>,
}

impl<'a> Basin<'a> {
    fn size(&self) -> usize {
        self.members.len()
    }
}

#[derive(Debug, Clone)]
struct Grid<T>
where
    T: Eq,
{
    cells: Vec<Cell<T>>,
    width: usize,
}

impl<T> Display for Grid<T>
where
    T: Display,
    T: Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut acc = String::new();
        for line in self.cells.chunks_exact(self.width) {
            for cell in line {
                acc.push_str(&format!("{}", cell.value));
            }
            acc.push('\n');
        }
        write!(f, "{}", acc.trim_end())
    }
}

impl Grid<u8> {
    fn coords_to_pos(&self, coords: (usize, usize)) -> usize {
        coords.0 + coords.1 * self.width
    }

    fn get(&self, coords: (usize, usize)) -> Option<&Cell<u8>> {
        if coords.0 < self.width {
            let pos = self.coords_to_pos(coords);
            self.cells.get(pos)
        } else {
            None
        }
    }

    #[allow(dead_code)] // This was used to help diagnose issues with basin identification
    fn display_basin(&self, basin: &Basin) -> String {
        self.cells
            .chunks_exact(self.width)
            .into_iter()
            .map(|row| -> String {
                row.iter()
                    .map(|cell| {
                        if basin.members.contains(&cell) {
                            cell.value.to_string()
                        } else {
                            ".".to_string()
                        }
                    })
                    .join("")
            })
            .join("\n")
    }

    fn basins<'a, 'b>(&'a self) -> Vec<Basin>
    where
        'b: 'a,
    {
        let mut pool: Vec<&Cell<u8>> = self.cells.iter().filter(|cell| cell.value != 9).collect();
        let mut seen: HashSet<&Cell<u8>> = HashSet::new();

        let mut basins = Vec::new();
        loop {
            let mut members: Vec<&Cell<u8>> = Vec::new();
            let head = match pool.pop() {
                None => break,
                Some(cell) => match !seen.contains(cell) {
                    true => cell,
                    false => continue,
                },
            };

            let mut pending: Vec<&Cell<u8>> = vec![head];
            while let Some(cell) = pending.pop() {
                // If pending is not empty
                // Then pop the tail and push it onto members
                members.push(cell);
                // Mark that member as seen
                seen.insert(cell);
                // Get the non-nine-value neighbors
                for neighbor in cell
                    .get_neighbors(self)
                    .into_iter()
                    .filter(|&neighbor| neighbor.value != 9)
                {
                    // and if they haven't been seen already
                    if !seen.contains(neighbor) {
                        // Push them into the pending list
                        pending.push(neighbor);
                        // and "see" them
                        seen.insert(neighbor);
                    }
                }
                // If pending IS empty
            }
            basins.push(Basin { members });
        }

        basins
    }
}

#[derive(Clone, Debug)]
struct Input<T: Eq> {
    grid: Grid<T>,
}

impl FromStr for Input<u8> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s.lines().flat_map(|line| {
            line.chars().map(|c| -> u8 {
                c.to_digit(10)
                    .expect("failed to parse char as base10 digit")
                    .try_into()
                    .expect("failed to convert u32 to u8")
            })
        });
        let width: usize = s.lines().next().unwrap().len();
        let coords = iproduct!((0..width), (0..width));
        let cells = coords
            .zip(cells)
            .map(|((y, x), value)| Cell {
                value,
                coords: (x, y),
            })
            .collect();
        Ok(Self {
            grid: Grid { cells, width },
        })
    }
}

const INPUT: &str = include_str!("input.txt");

fn solve_part1(input: Input<u8>) -> u64 {
    let grid = input.grid;
    grid.cells
        .iter()
        // If neighbors are all larger than self, then gimme that value as u64
        .filter_map(|cell| -> Option<u64> {
            let value = cell.value;
            let neighbors = cell.get_neighbors(&grid);
            if neighbors.iter().all(|neighbor| neighbor.value > value) {
                Some(cell.get_risk_level())
            } else {
                None
            }
        })
        .collect::<Vec<u64>>()
        .into_iter()
        // And sum them up
        .sum()
}

fn solve_part2(input: Input<u8>) -> u64 {
    let grid = input.grid;
    let mut basins = grid.basins();
    basins.sort_unstable_by_key(|basin| basin.size());
    basins
        .into_iter()
        .rev()
        .take(3)
        .map(|basin| -> u64 { basin.size().try_into().unwrap() })
        .product()
}

fn main() {
    let input = INPUT.parse::<Input<u8>>().expect("Failed to parse input");
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
        let input = INPUT.parse::<Input<u8>>().unwrap();
        let part1 = super::solve_part1(input);
        let expected = 15;

        assert_eq!(part1, expected);
    }

    #[test]
    fn solve_part2() {
        let input = INPUT.parse::<Input<u8>>().unwrap();
        let part2 = super::solve_part2(input);
        let expected = 1134;

        assert_eq!(part2, expected);
    }
}
