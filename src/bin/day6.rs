use std::fs::read_to_string;

use aoc_2025::surrounded;
use itertools::Itertools;
use winnow::{
    Parser, Result,
    ascii::{dec_uint, line_ending, space0, space1},
    combinator::{alt, opt, repeat, separated, terminated},
    error::{StrContext, StrContextValue},
    stream::AsChar,
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

    println!("part 1: {}", sum_solve(&parse_1.parse(&i).unwrap()));
    println!("part 2: {}", sum_solve(&parse_2(&i)));
}

fn sum_solve(i: &Input) -> u64 {
    i.problems.iter().map(solve).sum()
}

fn solve(p: &Problem) -> u64 {
    match p.op {
        Op::Plus => p.numbers.iter().sum(),
        Op::Times => p.numbers.iter().product(),
    }
}

fn parse_1(i: &mut &str) -> Result<Input> {
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

fn parse_2(i: &str) -> Input {
    let nlines = i.lines().count();
    let linelen = i.lines().next().unwrap().chars().count();

    assert!(i.lines().map(|l| l.len()).all_equal());

    let cols = (0..linelen).map(|n| i.lines().map(move |l| l.chars().nth(n).unwrap()));

    let chunks = cols.rev().chunk_by(|key| !key.clone().all(|c| c == ' '));
    let chunks = chunks
        .into_iter()
        .filter_map(|(key, group)| key.then_some(group));

    let problems = chunks.map(|group| {
        let (g1, g2) = group.tee();

        (
            g1.take(nlines - 1).map(|col| {
                let (col, ass) = col.tee();
                assert_eq!(ass.count(), nlines);

                col.take(nlines - 1)
                    .filter_map(|c| c.to_digit(10).map(u64::from))
                    .fold(0, |acc, x| acc * 10 + x)
            }),
            g2.map(|c| c.last().unwrap())
                .filter(|c| !c.is_space())
                .next()
                .unwrap(),
        )
    });

    Input {
        problems: problems
            .map(|(nums, op)| Problem {
                numbers: nums.collect(),
                op: match op {
                    '+' => Op::Plus,
                    '*' => Op::Times,
                    _ => unreachable!(),
                },
            })
            .collect(),
    }
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
    fn test_parse_1() {
        assert_eq!(
            parse_1.parse(INPUT),
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
    fn test_parse_2() {
        assert_eq!(
            parse_2(INPUT),
            Input {
                problems: vec![
                    Problem {
                        numbers: vec![4, 431, 623],
                        op: Op::Plus
                    },
                    Problem {
                        numbers: vec![175, 581, 32],
                        op: Op::Times
                    },
                    Problem {
                        numbers: vec![8, 248, 369],
                        op: Op::Plus
                    },
                    Problem {
                        numbers: vec![356, 24, 1],
                        op: Op::Times
                    },
                ]
            }
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
        assert_eq!(sum_solve(&parse_1.parse(INPUT).unwrap()), 4277556);
    }
}
