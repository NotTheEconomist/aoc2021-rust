use std::{cmp::Ordering, collections::HashMap, fmt::Display};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn step_towards(&self, other: &Point) -> Point {
        let x = match self.x.cmp(&other.x) {
            Ordering::Less => self.x + 1,
            Ordering::Equal => self.x,
            Ordering::Greater => self.x - 1,
        };
        let y = match self.y.cmp(&other.y) {
            Ordering::Less => self.y + 1,
            Ordering::Equal => self.y,
            Ordering::Greater => self.y - 1,
        };
        Self { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({}, {})", self.x, self.y))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum DiagonalHandling {
    Ignore,
    Include,
}

#[derive(Debug, Eq, PartialEq)]
struct Line(Vec<Point>);

impl Line {
    #[allow(unused)]
    fn between(start: Point, end: Point, handling: DiagonalHandling) -> Self {
        let mut result: Vec<Point> = Vec::new();

        // Check for valid input, rejecting non-straight lines and non-45 degrees
        match handling {
            // Early out if the line isn't straight
            DiagonalHandling::Ignore => {
                if start.x != end.x && start.y != end.y {
                    return Self(Vec::new());
                }
            }

            // Early out if the line isn't straight or if the diagonal isn't
            // 45 degrees
            DiagonalHandling::Include => {
                if (start.x != end.x && start.y != end.y)
                    && (start.x.abs_diff(end.x) != start.y.abs_diff(end.y))
                {
                    return Self(Vec::new());
                }
            }
        }

        // Make sure that we're always going from the smallest to the largest
        let mut point = start;
        result.push(point);
        loop {
            point = point.step_towards(&end);
            result.push(point);
            if point == end {
                break;
            }
        }
        Self(result)
    }
}

impl IntoIterator for Line {
    type Item = Point;

    type IntoIter = std::vec::IntoIter<Point>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Input {
    map: HashMap<Point, u32>,
}

impl Input {
    fn parse_with_handling(
        input: &'static str,
        handling: DiagonalHandling,
    ) -> Result<Self, String> {
        let mut map: HashMap<Point, u32> = HashMap::new();
        let points = input.lines().flat_map(|line| {
            {
                let (start, end) = line
                    .split_once(" -> ")
                    .expect("input line does not contain \" -> \"");
                let (x, y) = start
                    .split_once(',')
                    .expect("start atom doesn't look like \"x,y\"");
                let start = Point {
                    x: x.parse().expect("start's x did not parse"),
                    y: y.parse().expect("start's y did not parse"),
                };
                let (x, y) = end
                    .split_once(',')
                    .expect("start atom doesn't look like \"x,y\"");
                let stop = Point {
                    x: x.parse().expect("end's x did not parse"),
                    y: y.parse().expect("end's y did not parse"),
                };
                Line::between(start, stop, handling)
            }
            .into_iter()
        });
        for point in points {
            map.entry(point)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        Ok(Input { map })
    }
    fn parse(input: &'static str) -> Result<Self, String> {
        Self::parse_with_handling(input, DiagonalHandling::Ignore)
    }
}

fn solve_part1(input: Input) -> u32 {
    input.map.values().fold(
        0,
        |acc, point_count| if *point_count > 1 { acc + 1 } else { acc },
    )
}

fn solve_part2(input: Input) -> u32 {
    input.map.values().fold(
        0,
        |acc, point_count| if *point_count > 1 { acc + 1 } else { acc },
    )
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = Input::parse(INPUT).expect("Failed to parse input");
    let part1 = solve_part1(input);
    println!("part1: {}", part1);
    let part2_input = Input::parse_with_handling(INPUT, DiagonalHandling::Include)
        .expect("Failed to parse input");
    let part2 = solve_part2(part2_input);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_solve_part1() {
        let got = solve_part1(Input::parse(INPUT).expect("Could not parse input"));
        let expect = 5;
        assert_eq!(got, expect);
    }

    #[test]
    fn test_solve_part2() {
        let got = solve_part2(
            Input::parse_with_handling(INPUT, DiagonalHandling::Include)
                .expect("Could not parse intput"),
        );
        let expect = 12;
        assert_eq!(got, expect);
    }

    #[test]
    fn parse_input() {
        let input: &'static str = "\
1,1 -> 1,3
1,1 -> 3,1";

        let got = Input::parse(input).expect("Cannot parse input");
        let expect = Input {
            map: HashMap::<Point, u32>::from_iter(
                [
                    (Point { x: 1, y: 1 }, 2),
                    (Point { x: 1, y: 2 }, 1),
                    (Point { x: 1, y: 3 }, 1),
                    (Point { x: 2, y: 1 }, 1),
                    (Point { x: 3, y: 1 }, 1),
                ]
                .into_iter(),
            ),
        };
        assert_eq!(got, expect);
    }

    #[test]
    fn build_backwards_line() {
        let line = Line::between(
            Point { x: 3, y: 8 },
            Point { x: 3, y: 3 },
            DiagonalHandling::Ignore,
        );
        let expect = vec![
            Point { x: 3, y: 8 },
            Point { x: 3, y: 7 },
            Point { x: 3, y: 6 },
            Point { x: 3, y: 5 },
            Point { x: 3, y: 4 },
            Point { x: 3, y: 3 },
        ];

        assert_eq!(line, Line(expect));
    }
    #[test]
    fn build_line() {
        let line = Line::between(
            Point { x: 3, y: 3 },
            Point { x: 3, y: 8 },
            DiagonalHandling::Ignore,
        );
        let expect = vec![
            Point { x: 3, y: 3 },
            Point { x: 3, y: 4 },
            Point { x: 3, y: 5 },
            Point { x: 3, y: 6 },
            Point { x: 3, y: 7 },
            Point { x: 3, y: 8 },
        ];

        assert_eq!(line, Line(expect));
    }
}
