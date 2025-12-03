use std::fs;

use winnow::{
    self as w, Parser,
    ascii::line_ending,
    combinator::{repeat, terminated},
    token::one_of,
};

fn main() {
    let i = fs::read_to_string("input/day3").unwrap();
    let i = parse.parse(&mut &i).unwrap();

    println!("part 1: {}", part_1(&i).sum::<u64>());
}

fn part_1(i: &Vec<Vec<u64>>) -> impl Iterator<Item = u64> {
    i.iter().map(biggest_joltage)
}

fn biggest_joltage(i: &Vec<u64>) -> u64 {
    let mut tens = 0;
    let mut unit = 0;

    let mut it = i.iter().peekable();

    while let Some(&n) = it.next() {
        if n <= tens && n > unit {
            unit = n;
        } else if n > tens {
            match it.peek() {
                Some(&p) => {
                    unit = dbg!(*p);
                    tens = n;
                }
                None => unit = n.max(unit),
            }
        }
    }

    tens * 10 + unit
}

fn parse(s: &mut &str) -> w::Result<Vec<Vec<u64>>> {
    repeat(1.., terminated(parse_bank, line_ending)).parse_next(s)
}

fn parse_bank(s: &mut &str) -> w::Result<Vec<u64>> {
    repeat(
        1..,
        one_of('0'..='9').map(|c: char| u64::from(c.to_digit(10).unwrap())),
    )
    .parse_next(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use winnow::Parser;

    const INPUT: &str = concat!(
        "987654321111111\n",
        "811111111111119\n",
        "234234234234278\n",
        "818181911112111\n",
    );

    #[test]
    fn test_parse() {
        assert_eq!(
            parse.parse("123\n456\n"),
            Ok(vec![vec![1, 2, 3], vec![4, 5, 6]])
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(&parse.parse(INPUT).unwrap()).collect::<Vec<_>>(),
            vec![98, 89, 78, 92]
        );
    }

    #[test]
    fn test_joltage() {
        assert_eq!(biggest_joltage(&vec![9, 1, 9]), 99);
        assert_eq!(biggest_joltage(&vec![1, 4, 2]), 42);
        assert_eq!(biggest_joltage(&vec![1, 2, 3]), 23);
    }
}
