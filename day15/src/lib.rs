use std::str::FromStr;

use petgraph::IntoWeightedEdge;

#[derive(Debug, Copy, Hash, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
    pub value: u32,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.y.partial_cmp(&other.y) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.x.partial_cmp(&other.x) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // safety: there are no u32s that are not Ord
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge((Point, Point));

impl IntoWeightedEdge<u32> for Edge {
    type NodeId = Point;

    fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, u32) {
        let (from, to) = self.0;
        let weight = *&to.value;
        (from, to, weight)
    }
}

impl Edge {
    fn new(a: Point, b: Point) -> Self {
        Self((a, b))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input(Vec<Point>);

impl Input {
    fn get_point(&self, x: u32, y: u32) -> Option<&Point> {
        self.0.iter().find(|&point| point.x == x && point.y == y)
    }

    /// Width as a 1-indexed usize
    fn get_width(&self) -> usize {
        self.0
            .iter()
            .map(|point| point.x as usize + 1)
            .max()
            .unwrap_or(0)
    }

    /// Height as a 1-indexed usize
    fn get_height(&self) -> usize {
        self.0
            .iter()
            .map(|point| point.y as usize + 1)
            .max()
            .unwrap_or(0)
    }

    pub fn into_edges(self) -> Vec<Edge> {
        self.0
            .iter()
            .flat_map(|point| {
                [
                    self.get_point(point.x, point.y + 1),
                    self.get_point(point.x + 1, point.y),
                ]
                .map(|dest| -> Option<Edge> {
                    if let Some(dest_point) = dest.map(|point| point.clone()) {
                        Some(Edge::new(point.clone(), dest_point))
                    } else {
                        None
                    }
                })
                .into_iter()
                .flatten()
            })
            .collect()
    }

    pub fn scale(&mut self, times: usize) {
        let height = self.get_height();
        let width = self.get_width();
        let mut new_points = Vec::new();
        for &point in self.0.iter() {
            for scalar_y in 0..times {
                for scalar_x in 0..times {
                    // If both scalars are 0, that's just the original point
                    if scalar_y == 0 && scalar_x == 0 {
                        continue;
                    }
                    let new_point = Point {
                        x: point.x + (width * scalar_x) as u32,
                        y: point.y + (height * scalar_y) as u32,
                        value: (point.value + scalar_x as u32 + scalar_y as u32 - 1) % 9 + 1,
                    };
                    new_points.push(new_point);
                }
            }
        }
        self.0.extend(new_points.into_iter());
    }
}
impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut acc = Vec::new();
        for (y, line) in (0..).zip(s.lines()) {
            for (x, ch) in (0..).zip(line.chars()) {
                let value = ch.to_digit(10).ok_or(())?;
                acc.push(Point { x, y, value });
            }
        }

        Ok(Input(acc))
    }
}

#[cfg(test)]
mod tests {
    use petgraph::{algo::dijkstra, prelude::*};

    use super::*;

    #[test]
    fn test_extend_input() {
        let input_values = "\
345
678";
        let expected_scaled_values = "\
345456
678789
456567
789891";
        let mut input = input_values.parse::<Input>().expect("Input must parse");
        assert_eq!(input.get_height(), 2);
        assert_eq!(input.get_width(), 3);
        assert_eq!(input.0.len(), 6);

        input.scale(2);
        assert_eq!(input.get_height(), 4);
        assert_eq!(input.get_width(), 6);
        assert_eq!(input.0.len(), 24);

        let mut expected = expected_scaled_values
            .parse::<Input>()
            .expect("Expected output must parse");
        input.0.sort();
        expected.0.sort();
        assert_eq!(input, expected);
    }

    #[test]
    fn test_edge_weighting() {
        let a = Point {
            x: 0,
            y: 0,
            value: 3,
        };
        let b = Point {
            x: 1,
            y: 0,
            value: 5,
        };
        let graph: GraphMap<Point, u32, _> = UnGraphMap::from_edges(&[(a, b)]);
        let one_way = dijkstra(&graph, a, Some(b), |(_, b, _)| b.value);
        let the_other_way = dijkstra(&graph, b, Some(a), |(_, b, _)| b.value);

        assert_ne!(one_way, the_other_way);
    }

    #[test]
    fn test_into_edges() {
        /*
         *  a b  =  1 2
         *  c d     3 4
         */
        let a = Point {
            x: 0,
            y: 0,
            value: 1,
        };
        let b = Point {
            x: 1,
            y: 0,
            value: 2,
        };
        let c = Point {
            x: 0,
            y: 1,
            value: 3,
        };
        let d = Point {
            x: 1,
            y: 1,
            value: 4,
        };
        let input = Input(vec![a.clone(), b.clone(), c.clone(), d.clone()]);

        for (got, expected) in input.into_edges().into_iter().zip(
            [
                Edge::new(a.clone(), c.clone()),
                Edge::new(a, b.clone()),
                Edge::new(b, d.clone()),
                Edge::new(c, d),
            ]
            .into_iter(),
        ) {
            assert_eq!(got, expected);
        }
    }
}
