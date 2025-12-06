use std::fs::read_to_string;

use aoc_2025::surrounded;
use winnow::{
    ModalResult, Parser, Result,
    ascii::{dec_uint, line_ending, space0, space1},
    combinator::{alt, opt, repeat, separated, terminated},
    error::{StrContext, StrContextValue},
};

#[derive(Debug, PartialEq, Eq)]
struct Input {
    problems: Vec<Problem>,
}

#[derive(Debug, PartialEq, Eq)]
struct Problem {
    numbers: Vec<u64>,
    op: Op,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    Plus,
    Times,
}

fn main() {
    let i = read_to_string("input/day6").unwrap();
    let i = parse.parse(&i).unwrap();

    println!("part 1: {}", part_1(&i));
}

fn part_1(i: &Input) -> u64 {
    i.problems.iter().map(solve).sum()
}

fn solve(p: &Problem) -> u64 {
    match p.op {
        Op::Plus => p.numbers.iter().sum(),
        Op::Times => p.numbers.iter().product(),
    }
}

fn parse(i: &mut &str) -> Result<Input> {
    let ns: Vec<_> = repeat(1.., terminated(number_line, line_ending)).parse_next(i)?;
    let ops = terminated(op_line, opt(line_ending)).parse_next(i)?;

    let len = ops.len();
    assert!(ns.iter().all(|e| e.len() == len));

    let mut r = Input {
        problems: Vec::with_capacity(len),
    };
    for i in 0..len {
        r.problems.push(Problem {
            numbers: ns.iter().map(|v| v[i]).collect(),
            op: ops[i],
        });
    }

    Ok(r)
}

fn number_line(i: &mut &str) -> Result<Vec<u64>> {
    surrounded(
        separated(
            1..,
            dec_uint::<_, u64, _>
                .context(StrContext::Expected(StrContextValue::Description("number"))),
            space1,
        ),
        space0,
    )
    .parse_next(i)
}

fn op_line(i: &mut &str) -> Result<Vec<Op>> {
    surrounded(
        separated(
            1..,
            alt(('+'.map(|_| Op::Plus), ('*'.map(|_| Op::Times))))
                .context(StrContext::Expected(StrContextValue::Description("number"))),
            space1,
        ),
        space0,
    )
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use winnow::Parser;

    const INPUT: &str = concat!(
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  \n",
    );

    #[test]
    fn test_parse() {
        let p = parse.parse(INPUT);
        assert_eq!(
            p,
            Ok(Input {
                problems: vec![
                    Problem {
                        numbers: vec![123, 45, 6],
                        op: Op::Times
                    },
                    Problem {
                        numbers: vec![328, 64, 98],
                        op: Op::Plus
                    },
                    Problem {
                        numbers: vec![51, 387, 215],
                        op: Op::Times
                    },
                    Problem {
                        numbers: vec![64, 23, 314],
                        op: Op::Plus
                    },
                ]
            })
        );
    }

    #[test]
    fn test_parse_number_line() {
        assert_eq!(
            number_line.parse(" 45 64  387 23 "),
            Ok(vec![45, 64, 387, 23])
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse.parse(INPUT).unwrap()), 4277556);
    }
}
