use std::{collections::HashSet, fmt::Display, num::ParseIntError, str::FromStr};

const INPUT: &str = "\
1326253315
3427728113
5751612542
6543868322
4422526221
2234325647
1773174887
7281321674
6562513118
4824541522";

#[derive(Debug, PartialEq, Eq)]
struct OctopusCavern {
    octopuses: Vec<u16>,
    width: usize,
}
impl OctopusCavern {
    fn new(input: Input) -> Self {
        Self {
            octopuses: input.values,
            width: input.width,
        }
    }
    fn step(&mut self) -> usize {
        let mut flashes: HashSet<usize> = HashSet::new();
        self.octopuses.iter_mut().for_each(|n| *n += 1);
        loop {
            let mut flash_idxs: HashSet<usize> = HashSet::new();
            // Find the octopuses which are ready to flash
            for (i, &n) in self.octopuses.iter().enumerate() {
                if n >= 10 {
                    // Make sure they aren't already flashing
                    if !flashes.contains(&i) {
                        // And insert it into this round of flashers
                        flash_idxs.insert(i);
                    }
                }
            }

            // Once we know what's flashing this round, push them all into the step-wide set
            flashes.extend(flash_idxs.iter());
            if flash_idxs.is_empty() {
                // If there aren't any more flashing octopuses this round, we're done
                break;
            } else {
                // Otherwise, light up the surrounding square of each flasher
                for idx in flash_idxs.into_iter() {
                    self.octopuses[idx] = 0;
                    for neighbor_idx in self.get_neighbor_idxs(idx) {
                        if !flashes.contains(&neighbor_idx) {
                            self.octopuses[neighbor_idx] += 1;
                        }
                    }
                }
            }
        }
        flashes.len()
    }
    fn get_neighbor_idxs(&self, idx: usize) -> Vec<usize> {
        let mut indices: Vec<usize> = Vec::new();
        if let Some(top_left) = idx.checked_sub(self.width + 1) {
            // Check if idx is on the left edge
            if idx % self.width != 0 {
                indices.push(top_left);
            }
        }
        if let Some(top) = idx.checked_sub(self.width) {
            indices.push(top);
        }
        if let Some(top_right) = idx.checked_sub(self.width - 1) {
            // Check if idx is on the right edge
            if (idx + 1) % self.width != 0 {
                indices.push(top_right);
            }
        }
        if let Some(left) = idx.checked_sub(1) {
            // Check if idx is on the left edge
            if idx % self.width != 0 {
                indices.push(left);
            }
        }
        if let Some(right) = idx.checked_add(1) {
            // Check if idx is on the right edge
            if (idx + 1) % self.width != 0 {
                indices.push(right);
            }
        }
        if let Some(bottom_left) = idx.checked_add(self.width - 1) {
            // Check if idx is on the left edge
            if idx % self.width != 0 {
                // Check if idx is on the bottom edge
                if idx < 90 {
                    indices.push(bottom_left);
                }
            }
        }
        if let Some(bottom) = idx.checked_add(self.width) {
            // Check if idx is on the bottom edge
            if idx < 90 {
                indices.push(bottom);
            }
        }
        if let Some(bottom_right) = idx.checked_add(self.width + 1) {
            // Check if idx is on the right edge
            if (idx + 1) % self.width != 0 {
                // Check if idx is on the bottom edge
                if idx < 90 {
                    indices.push(bottom_right);
                }
            }
        }
        indices
    }
}
impl Iterator for OctopusCavern {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.step())
    }
}
impl Default for OctopusCavern {
    fn default() -> Self {
        Self {
            octopuses: Vec::new(),
            width: 10,
        }
    }
}
impl Display for OctopusCavern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .octopuses
            .chunks_exact(self.width)
            .map(|chunk| -> String {
                chunk
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", s)
    }
}

#[derive(Clone)]
struct Input {
    values: Vec<u16>,
    width: usize,
}
impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap_or("").len();
        let values = s
            .lines()
            .flat_map(|line| line.chars().map(|ch| ch.to_string().parse()))
            .collect::<Result<Vec<_>, ParseIntError>>()
            .map_err(|_| "Failed to parse a character from the input".to_string())?;
        match values.try_into() {
            Ok(values) => Ok(Self { values, width }),
            Err(_) => Err("Input has wrong number of elements".to_string()),
        }
    }
}

fn solve_part1(input: Input) -> u64 {
    let mut game = OctopusCavern::new(input);
    (0..100).fold(0, |acc, _| acc + game.step()) as u64
}

fn solve_part2(input: Input) -> u64 {
    let game = OctopusCavern::new(input);
    let game_width = game.width.clone();
    (1u64..)
        .zip(game.into_iter())
        .filter_map(|(i, flashes)| {
            if flashes == game_width * game_width {
                Some(i)
            } else {
                None
            }
        })
        .next()
        .unwrap() as u64
}

fn main() {
    let input: Input = INPUT.parse().expect("failed to parse input");
    let part1 = solve_part1(input.clone());
    println!("part1: {}", part1);
    let part2 = solve_part2(input);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod test {
    use crate::OctopusCavern;

    const INPUT: &'static str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn solve_part1() {
        let input = INPUT.parse().expect("Failed to parse input");
        let part1 = super::solve_part1(input);
        let expected = 1656;
        assert_eq!(part1, expected);
    }

    #[test]
    fn solve_part2() {
        let input = INPUT.parse().expect("Failed to parse input");
        let part2 = super::solve_part2(input);
        let expected = 195;
        assert_eq!(part2, expected);
    }

    #[test]
    fn assert_force_flashes() {
        let mut game = OctopusCavern::new(
            "\
11111
19991
19191
19991
11111"
                .parse()
                .unwrap(),
        );
        game.step();
        let expected = OctopusCavern::new(
            "\
34543
40004
50005
40004
34543"
                .parse()
                .unwrap(),
        );
        assert_eq!(game, expected);
    }

    #[test]
    fn step_once() {
        let input = INPUT.parse().expect("Failed to parse input");
        let mut game = OctopusCavern::new(input);
        game.step();
        {
            let expected = OctopusCavern::new(
                "\
6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637"
                    .parse()
                    .unwrap(),
            );

            assert_eq!(game, expected);
        }

        {
            let expected = OctopusCavern::new(
                "\
8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848"
                    .parse()
                    .unwrap(),
            );

            let num_flashes = game.step();

            assert_eq!(num_flashes, 35);
            assert_eq!(game, expected);
        }
    }
}
