use lazy_regex::regex;
use std::str::FromStr;

use day17::*;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Debug)]
struct Input {
    target_zone: TargetZone,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pat = regex!(
            r#"target area: x=(?P<x_min>-?\d+)\.\.(?P<x_max>-?\d+), y=(?P<y_min>-?\d+)\.\.(?P<y_max>-?\d+)"#
        );
        let captures = pat
            .captures(s)
            .ok_or_else(|| String::from("Couldn't parse input"))?;
        let x_min: i32 = captures
            .name("x_min")
            .ok_or_else(|| String::from("Couldn't find x_min for target zone"))?
            .as_str()
            .parse()
            .map_err(|_| String::from("x_min must be a valid i32"))?;
        let y_min: i32 = captures
            .name("y_min")
            .ok_or_else(|| String::from("Couldn't find y_min for target zone"))?
            .as_str()
            .parse()
            .map_err(|_| String::from("y_min must be a valid i32"))?;
        let x_max: i32 = captures
            .name("x_max")
            .ok_or_else(|| String::from("Couldn't find x_max for target zone"))?
            .as_str()
            .parse()
            .map_err(|_| String::from("x_max must be a valid i32"))?;
        let y_max: i32 = captures
            .name("y_max")
            .ok_or_else(|| String::from("Couldn't find y_max for target zone"))?
            .as_str()
            .parse()
            .map_err(|_| String::from("y_max must be a valid i32"))?;

        let target_zone = TargetZone {
            bottom_left: Point { x: x_min, y: y_min },
            top_right: Point { x: x_max, y: y_max },
        };
        Ok(Self { target_zone })
    }
}

fn vector_x_bounds(target_zone: &TargetZone) -> (i32, i32) {
    // solution to quadratic n^2 + n - target_zone.bottom_left.x * 2
    let dx_min: i32 = ((-1.0 + (1.0 - (-4.0 * target_zone.bottom_left.x as f32 * 2.0)).sqrt())
        / 2.0)
        .round() as i32;
    // solution to quadratic n^2 + n - target_zone.top_right.x * 2
    let dx_max: i32 = ((-1.0 + (1.0 - (-4.0 * target_zone.top_right.x as f32 * 2.0)).sqrt()) / 2.0)
        .round() as i32;
    (dx_min, dx_max)
}

fn vector_x_bounds_extreme(target_zone: &TargetZone) -> (i32, i32) {
    let (dx_min, _) = vector_x_bounds(target_zone);
    let dx_max = target_zone.top_right.x;
    (dx_min, dx_max)
}

fn calculate_hit(target_zone: &TargetZone, mut vector: Vector) -> bool {
    let mut pos = Point { x: 0, y: 0 };
    // rise until our peak
    while !has_past(&pos, &vector, target_zone) {
        if target_zone.contains(&pos) {
            return true;
        }
        if pos.try_apply_vector(&mut vector).is_err() {
            return false;
        }
    }
    false
}

/// Given a value dx, find all values dy to complete (dx, dy) such that
/// the projectile will cross into target_zone
fn vector_find_hits(target_zone: &TargetZone, dx: i32) -> Vec<Vector> {
    // start guessing ys
    // if the target zone is below (0, 0) as ours is, we are guaranteed that any
    // dy > 0 will arc parabolically up and return down to (_, 0) with a velocity
    // of (_, -dy)
    // Because of this fact, any initial dy greater than abs(target_zone.bottom_left.y)
    // will fall beyond the bottom of the target zone on the first tick after it
    // reaches the center line again. Since every dy will eventually reach (_, 0)
    // that can serve as our hard upper limit.
    (target_zone.bottom_left.y..=-target_zone.bottom_left.y)
        // skip until we start getting hits
        .skip_while(|&dy| !calculate_hit(target_zone, Vector { x: dx, y: dy }))
        .filter_map(|dy| {
            let vector = Vector { x: dx, y: dy };
            if calculate_hit(target_zone, vector) {
                Some(vector)
            } else {
                None
            }
        })
        .collect()
}

fn solve_part2(input: Input) -> u64 {
    let target_zone = input.target_zone;

    let (dx_min, dx_max) = vector_x_bounds_extreme(&target_zone);
    (dx_min..=dx_max)
        .flat_map(|dx| vector_find_hits(&target_zone, dx))
        .count() as u64
}

fn solve_part1(input: Input) -> u64 {
    let target_zone = input.target_zone;

    let (dx_min, dx_max) = vector_x_bounds_extreme(&target_zone);
    let best_dy = (dx_min..=dx_max)
        .flat_map(|dx| vector_find_hits(&target_zone, dx))
        .map(|vector| vector.y)
        .max()
        .expect("There must be some vector that hits");

    (1..=best_dy).fold(0, |acc, next| acc + next as u64)
}

fn main() {
    let input = INPUT.parse::<Input>().expect("Input must parse");
    let part1 = solve_part1(input.clone());
    let part2 = solve_part2(input);

    println!("part1: {part1}\npart2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");
    #[test]
    fn solve_part2() {
        let input = INPUT.parse().expect("Input must parse");
        let part2 = super::solve_part2(input);

        assert_eq!(part2, 112)
    }
    #[test]
    fn solve_part1() {
        let input = INPUT.parse::<Input>().expect("Input must parse");
        let part1 = super::solve_part1(input);

        assert_eq!(part1, 45);
    }
}

#[cfg(test)]
mod invariants {
    use super::*;
    fn triangle_sum(n: i32) -> i32 {
        // trivially (1..=n).sum(), but mathematically this generalizes to:
        n * (n + 1) / 2
    }
    #[test]
    fn calculating_dx_max_and_min() {
        let target_zone = TargetZone {
            bottom_left: Point { x: 20, y: -10 },
            top_right: Point { x: 30, y: -5 },
        };
        let (dx_min, dx_max) = vector_x_bounds(&target_zone);
        assert_eq!((dx_min, dx_max), (6, 7));

        dbg!(vector_x_bounds(&TargetZone {
            bottom_left: Point { x: 265, y: -103 },
            top_right: Point { x: 287, y: -58 },
        }));

        for dx in dx_min..=dx_max {
            let max_x = triangle_sum(dx);
            assert!(
                target_zone.bottom_left.x <= max_x && max_x <= target_zone.top_right.x,
                "dx_min={}, dx_max={}, max_x={}, dx={}",
                dx_min,
                dx_max,
                max_x,
                dx
            );
        }
    }

    #[test]
    fn calculate_dy_from_given_dx() {
        let target_zone = TargetZone {
            bottom_left: Point { x: 20, y: -10 },
            top_right: Point { x: 30, y: -5 },
        };

        let possible_dys = vector_find_hits(&target_zone, 6);
        let expected: Vec<Vector> = (0..=9)
            .into_iter()
            .map(|dy| Vector { x: 6, y: dy })
            .collect();
        assert_eq!(possible_dys, expected);
        let possible_dys = vector_find_hits(&target_zone, 7);
        let expected: Vec<Vector> = (-1..=9)
            .into_iter()
            .map(|dy| Vector { x: 7, y: dy })
            .collect();
        assert_eq!(possible_dys, expected);
    }
}
