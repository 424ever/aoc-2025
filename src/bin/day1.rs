use std::fs;

use winnow::{
    Parser,
    ascii::newline,
    combinator::{opt, repeat},
    token::{one_of, take_while},
};

fn main() -> anyhow::Result<()> {
    let i = fs::read_to_string("input/day1")?;
    let i = parse.parse(&mut &i).unwrap();

    println!("part 1: {}", part_1(&i));
    println!("part 2: {}", part_2(&i));
    Ok(())
}

fn part_1(input: &Vec<i32>) -> u32 {
    let mut cur = 50i32;
    let mut count = 0;

    for i in input {
        cur += i;

        while cur < 0 {
            cur = 100 + cur
        }

        while cur > 99 {
            cur = cur - 100
        }

        if cur == 0 {
            count += 1
        }
    }

    count
}

fn part_2(input: &Vec<i32>) -> u32 {
    let mut cur = 50i32;
    let mut count = 0;

    for mut i in input.iter().cloned() {
        while i < 0 {
            i += 1;
            cur += 1;
            if cur == 100 {
                cur = 0
            }
            if cur == 0 {
                count += 1
            }
        }

        while i > 0 {
            i -= 1;
            cur -= 1;
            if cur == -1 {
                cur = 99
            }
            if cur == 0 {
                count += 1
            }
        }
    }

    count
}

fn parse(input: &mut &str) -> winnow::Result<Vec<i32>> {
    repeat(0.., (parse_line, opt(newline)).map(|l| l.0)).parse_next(input)
}

fn parse_line(s: &mut &str) -> winnow::Result<i32> {
    let dir = one_of(('L', 'R')).parse_next(s)?;
    let dig = take_while(1.., '0'..='9').parse_next(s)?;

    let num: i32 = dig.parse().expect("number");

    Ok(match dir {
        'L' => -num,
        'R' => num,
        _ => unreachable!(),
    })
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1, part_2};
    use winnow::Parser;

    const INPUT: &str = concat!(
        "L68\n", "L30\n", "R48\n", "L5\n", "R60\n", "L55\n", "L1\n", "L99\n", "R14\n", "L82\n",
    );

    #[test]
    fn test_parse() {
        assert_eq!(
            parse.parse(INPUT).unwrap(),
            vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse.parse(INPUT).unwrap()), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse.parse(INPUT).unwrap()), 6);
    }
}
