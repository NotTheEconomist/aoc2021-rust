use std::cmp::Ordering;
use std::iter::IntoIterator;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input: Vec<i32> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    let part1 = solve_part1(input.clone());
    println!("part1: {}", part1);
    let part2 = solve_part2(input);
    println!("part2: {}", part2);
}

fn solve_part1(input: Vec<i32>) -> i32 {
    let (a, mut b) = (input.clone().into_iter(), input.into_iter());
    b.next();
    let pairs = Iterator::zip(a, b);
    pairs.fold(0, |acc, (prev, next)| {
        if let Ordering::Greater = next.cmp(&prev) {
            acc + 1
        } else {
            acc
        }
    })
}

struct SumTriple<S, T>
where
    S: Iterator<Item = T>,
    T: Copy,
{
    iter: S,
    prev: T,
    prev_prev: T,
}

impl<T> TryFrom<Vec<T>> for SumTriple<std::vec::IntoIter<T>, T>
where
    T: Copy,
{
    type Error = String;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        let mut iter = value.into_iter();
        let prev_prev = iter
            .next()
            .ok_or("SumTriple members must have length 3 or longer")?;
        let prev = iter
            .next()
            .ok_or("SumTriple members must have length 3 or longer")?;
        Ok(Self {
            iter,
            prev,
            prev_prev,
        })
    }
}

impl<S, T> Iterator for SumTriple<S, T>
where
    S: Iterator<Item = T>,
    T: Copy,
    T: std::ops::Add,
    T: std::ops::Add<Output = T>,
{
    type Item = <T as std::ops::Add>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        let result = next + self.prev + self.prev_prev;
        self.prev_prev = self.prev;
        self.prev = next;
        Some(result)
    }
}

fn solve_part2(input: Vec<i32>) -> i32 {
    let (a, mut b) = (
        SumTriple::try_from(input.clone()).unwrap(),
        SumTriple::try_from(input).unwrap(),
    );
    b.next();
    let pairs = a.zip(b);
    pairs.fold(0, |acc, (prev, next)| {
        if let Ordering::Greater = next.cmp(&prev) {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_solve_part1() {
        let input = INPUT.lines().map(|line| line.parse().unwrap()).collect();

        let expect = 7;
        assert_eq!(solve_part1(input), expect);
    }

    #[test]
    fn test_solve_part2() {
        let input = INPUT.lines().map(|line| line.parse().unwrap()).collect();

        let expect = 5;
        assert_eq!(solve_part2(input), expect);
    }
}
