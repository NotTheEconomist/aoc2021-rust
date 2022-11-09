use petgraph::algo::astar;
use petgraph::graphmap::UnGraphMap;

const INPUT: &str = include_str!("input.txt");

fn solve_part1(input: day15::Input) -> u64 {
    let graph = UnGraphMap::from_edges(input.into_edges());
    let start = graph
        .nodes()
        .find(|point| point.x == 0 && point.y == 0)
        .expect("(0, 0) must be contained in the graph");
    let end = graph
        .nodes()
        .reduce(|acc, point| {
            if (point.x, point.y) > (acc.x, acc.y) {
                point
            } else {
                acc
            }
        })
        .unwrap();

    let (distance, _) = astar(
        &graph,
        start,
        |point| point == end,
        |(_, dest, _)| dest.value,
        |point| end.y - point.y + end.x - point.x,
    )
    .expect("There must be a path from start to end");

    distance as u64
}

fn solve_part2(mut input: day15::Input) -> u64 {
    input.scale(5);

    solve_part1(input)
}

fn main() {
    let input = INPUT.parse::<day15::Input>().unwrap();
    let part1 = solve_part1(input.clone());
    println!("part1: {}", part1);
    let part2 = solve_part2(input);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn solve_part1() {
        let input = INPUT.parse::<day15::Input>().expect("Input must parse");

        let part1 = super::solve_part1(input);
        let expected = 40;
        assert_eq!(part1, expected);
    }
    #[test]
    fn solve_part2() {
        let input = INPUT.parse::<day15::Input>().expect("Input must parse");

        let part2 = super::solve_part2(input);
        let expected = 315;
        assert_eq!(part2, expected);
    }
}
