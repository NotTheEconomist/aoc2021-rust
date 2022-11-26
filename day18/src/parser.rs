use nom::IResult;

use crate::SnailFish;

pub(crate) fn inner(s: &str) -> IResult<&str, (SnailFish, SnailFish)> {
    nom::sequence::separated_pair(
        nom::branch::alt((
            nom::combinator::map(nom::character::complete::u32, SnailFish::num),
            outer,
        )),
        nom::sequence::pair(
            nom::character::complete::char(','),
            nom::character::complete::multispace0,
        ),
        nom::branch::alt((
            nom::combinator::map(nom::character::complete::u32, SnailFish::num),
            outer,
        )),
    )(s)
}

pub(crate) fn outer(s: &str) -> IResult<&str, SnailFish> {
    nom::combinator::map(
        nom::sequence::delimited(
            nom::bytes::complete::tag("["),
            inner,
            nom::bytes::complete::tag("]"),
        ),
        |(a, b)| SnailFish::pair(a, b),
    )(s)
}

pub(crate) fn root(s: &str) -> IResult<&str, SnailFish> {
    nom::combinator::all_consuming(outer)(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_pair() {
        let s = "[1, 2]";
        let (_, result) = root(s).unwrap();
        assert_eq!(
            result,
            SnailFish::pair(SnailFish::num(1), SnailFish::num(2))
        );
    }

    #[test]
    fn test_multiple_pairs() {
        let s = "[1, [2, [3, 4]]]";
        let (_, result) = root(s).unwrap();
        assert_eq!(
            result,
            SnailFish::pair(
                SnailFish::num(1),
                SnailFish::pair(
                    SnailFish::num(2),
                    SnailFish::pair(SnailFish::num(3), SnailFish::num(4),)
                )
            )
        );
    }
}
