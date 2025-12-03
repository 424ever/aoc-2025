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

    println!("part 1: {}", joltages(&i, 2).sum::<u64>());
    println!("part 2: {}", joltages(&i, 12).sum::<u64>());
}

fn joltages(i: &Vec<Vec<u8>>, digits: usize) -> impl Iterator<Item = u64> {
    i.iter().map(move |v| biggest_joltage(v, digits))
}

fn biggest_joltage(v: &Vec<u8>, dig: usize) -> u64 {
    let mut digits = vec![0u8; dig];
    let mut lasthi = None;

    for n in (0..dig).rev() {
        let from = if let Some(l) = lasthi { l + 1 } else { 0 };
        let to = v.len() - n;
        let m = v[from..to].iter().max().unwrap();
        let maxidx = v[from..].iter().position(|e| e == m).unwrap() + from;
        digits.push(v[usize::try_from(maxidx).unwrap()]);
        lasthi = Some(maxidx);
    }

    digits.iter().fold(0, |acc, &e| acc * 10 + u64::from(e))
}

fn parse(s: &mut &str) -> w::Result<Vec<Vec<u8>>> {
    repeat(1.., terminated(parse_bank, line_ending)).parse_next(s)
}

fn parse_bank(s: &mut &str) -> w::Result<Vec<u8>> {
    repeat(
        1..,
        one_of('0'..='9').map(|c: char| u8::try_from(c.to_digit(10).unwrap()).unwrap()),
    )
    .parse_next(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::proptest;
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
            joltages(&parse.parse(INPUT).unwrap(), 2).collect::<Vec<_>>(),
            vec![98, 89, 78, 92]
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            joltages(&parse.parse(INPUT).unwrap(), 12).collect::<Vec<_>>(),
            vec![987654321111, 811111111119, 434234234278, 888911112111]
        );
    }

    #[test]
    fn test_joltage() {
        assert_eq!(biggest_joltage(&vec![0, 0, 1, 1, 0], 4), 110);
        assert_eq!(biggest_joltage(&vec![9, 9, 9, 1, 9, 1, 1], 4), 9999);
        assert_eq!(biggest_joltage(&vec![9, 9, 9, 1, 9, 1, 1], 5), 99991);
        assert_eq!(biggest_joltage(&vec![9, 1, 9], 2), 99);
        assert_eq!(biggest_joltage(&vec![1, 4, 2], 2), 42);
        assert_eq!(biggest_joltage(&vec![1, 2, 3], 2), 23);
        assert_eq!(
            biggest_joltage(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12),
            987654321111
        );
        assert_eq!(biggest_joltage(&vec![9, 5, 5, 2, 9], 4), 9559);
    }

    proptest! {
    #[test]
    fn test_joltage_prop(s in "[1-9]{7,}") {
        fn brute(v: &Vec<u8>) -> u64 {
            let l = v.len();
            let mut max = 0;
            for a in 0..l - 3 {
                for b in a+1..l - 2 {
                    for c in b+1..l - 1 {
                        for d in c+1..l - 0 {
                            max = format!("{}{}{}{}", v[a], v[b], v[c], v[d])
                                .parse::<u64>()
                                .unwrap()
                                .max(max);
                        }
                    }
                }
            }

            max
        }

        let v = parse_bank.parse(&s).unwrap();

        assert_eq!(biggest_joltage(&v, 4), brute(&v));
    }
    }
}
