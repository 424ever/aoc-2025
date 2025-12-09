use std::fs::read_to_string;

use itertools::Itertools;
use winnow::{
    Parser, Result,
    ascii::{dec_uint, line_ending},
    combinator::{repeat, separated_pair, terminated},
};

type Pos = (u64, u64);

fn main() {
    let i = read_to_string("input/day9").unwrap();
    let i = parse.parse(&i).unwrap();

    println!("part 1: {}", part_1(&i));
}

fn part_1(i: &[Pos]) -> u64 {
    let l = i
        .iter()
        .tuple_combinations()
        .max_by_key(|(p1, p2)| area(p1, p2))
        .unwrap();
    area(l.0, l.1)
}

fn area(p1: &Pos, p2: &Pos) -> u64 {
    (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1)
}

fn parse(i: &mut &str) -> Result<Vec<Pos>> {
    repeat(
        1..,
        terminated(
            separated_pair(dec_uint::<_, u64, _>, ',', dec_uint::<_, u64, _>),
            line_ending,
        ),
    )
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = concat!(
        "7,1\n", "11,1\n", "11,7\n", "9,7\n", "9,5\n", "2,5\n", "2,3\n", "7,3\n",
    );

    #[test]
    fn test_parse() {
        assert_eq!(parse.parse("1,2\n4,5\n"), Ok(vec![(1, 2), (4, 5)]));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse.parse(INPUT).unwrap()), 50);
    }
}
