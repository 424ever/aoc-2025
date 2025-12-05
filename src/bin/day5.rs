use std::fs::read_to_string;
use std::ops::RangeInclusive;

use winnow::{
    Parser, Result,
    ascii::{dec_uint, line_ending},
    combinator::{opt, repeat, separated_pair, seq, terminated},
};

type Range = RangeInclusive<u64>;

#[derive(Debug, PartialEq, Eq)]
struct Input {
    fresh_ranges: Vec<Range>,
    available: Vec<u64>,
}

fn main() {
    let i = read_to_string("input/day5").unwrap();
    let i = parse.parse(&i).unwrap();

    println!("part 1: {}", part_1(&i));
    println!("part 2: {}", part_2(&i));
}

fn part_1(i: &Input) -> usize {
    i.available
        .iter()
        .filter(|a| i.fresh_ranges.iter().any(|r| r.contains(a)))
        .count()
}

fn part_2(i: &Input) -> usize {
    i.fresh_ranges.iter().map(|r| r.clone().count()).sum()
}

fn reduce_ranges(v: Vec<Range>) -> Vec<Range> {
    let mut n = v.clone();
    let mut i = 0;

    n.sort_by(|a, b| a.start().cmp(b.start()));

    while i < n.len() {
        let mut cur = &n[i];
        while let Some(next) = n.get(i + 1)
            && next.start() <= cur.end()
        {
            n[i] = *cur.start()..=*next.end().max(cur.end());
            n.remove(i + 1);
            cur = &n[i];
        }
        i += 1
    }

    n
}

fn parse(i: &mut &str) -> Result<Input> {
    seq! {Input {
        fresh_ranges: fresh_ranges.map(reduce_ranges),
        _: '\n',
        available: available
    }}
    .parse_next(i)
}

fn fresh_ranges(i: &mut &str) -> Result<Vec<Range>> {
    repeat(
        1..,
        terminated(
            separated_pair(dec_uint::<_, u64, _>, '-', dec_uint::<_, u64, _>),
            line_ending,
        )
        .map(|r| Range::new(r.0, r.1)),
    )
    .parse_next(i)
}

fn available(i: &mut &str) -> Result<Vec<u64>> {
    repeat(1.., terminated(dec_uint::<_, u64, _>, opt(line_ending))).parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use winnow::Parser;

    const INPUT: &str = concat!(
        "3-5\n", "10-14\n", "16-20\n", "12-18\n", "\n", "1\n", "5\n", "8\n", "11\n", "17\n",
        "32\n",
    );

    #[test]
    fn test_parse() {
        assert_eq!(
            parse.parse(INPUT),
            Ok(Input {
                fresh_ranges: vec![3..=5, 10..=20],
                available: vec![1, 5, 8, 11, 17, 32],
            })
        );
    }

    #[test]
    fn test_reduce() {
        assert_eq!(
            reduce_ranges(vec![154..=33, 204..=209, 153..=90, 132..=84, 10..=246]),
            vec![10..=246]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse.parse(INPUT).unwrap()), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse.parse(INPUT).unwrap()), 14);
    }
}
