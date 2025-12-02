use anyhow as a;
use winnow::{
    self as w, Parser,
    ascii::{dec_uint, newline},
    combinator::{opt, separated, separated_pair},
};

use std::{fs, ops::RangeInclusive};

fn main() -> a::Result<()> {
    let i = fs::read_to_string("input/day2")?;
    let i = parse.parse(&mut &i).unwrap();

    println!("part 1: {}", part_1(&i));
    Ok(())
}

fn part_1(i: &Vec<RangeInclusive<u64>>) -> u64 {
    i.iter().cloned().flatten().filter(repeated_twice).sum()
}

fn repeated_twice(n: &u64) -> bool {
    let s = format!("{}", n);

    if s.len() % 2 != 0 {
        return false;
    }

    let mid = s.len() >> 1;

    s[0..mid] == s[mid..]
}

fn parse(s: &mut &str) -> w::Result<Vec<RangeInclusive<u64>>> {
    Ok((separated(0.., parse_range, ','), opt(newline))
        .parse_next(s)?
        .0)
}

fn parse_range(s: &mut &str) -> w::Result<RangeInclusive<u64>> {
    let (n1, n2) = separated_pair(dec_uint, '-', dec_uint).parse_next(s)?;
    Ok(n1..=n2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use winnow::Parser;

    const INPUT: &str = concat!(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,",
        "1698522-1698528,446443-446449,38593856-38593862,565653-565659,",
        "824824821-824824827,2121212118-2121212124"
    );

    #[test]
    fn test_parse() {
        assert_eq!(
            parse.parse(INPUT).unwrap(),
            vec![
                11..=22,
                95..=115,
                998..=1012,
                1188511880..=1188511890,
                222220..=222224,
                1698522..=1698528,
                446443..=446449,
                38593856..=38593862,
                565653..=565659,
                824824821..=824824827,
                2121212118..=2121212124
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let i = parse.parse(INPUT).unwrap();
        assert_eq!(part_1(&i), 1227775554);
    }
}
