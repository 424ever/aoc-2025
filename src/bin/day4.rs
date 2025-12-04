use std::{collections::HashSet, fs};

type Pos = (usize, usize);

fn main() {
    let i = fs::read_to_string("input/day4").unwrap();
    let i = parse_input(&i);

    println!("part 1: {}", part_1(&i));
}

fn part_1(rolls: &HashSet<Pos>) -> usize {
    rolls
        .iter()
        .map(|r| adjacent(r, rolls).count())
        .filter(|&n| n < 4)
        .count()
}

fn adjacent(r: &Pos, all: &HashSet<Pos>) -> impl Iterator<Item = Pos> {
    (-1..=1)
        .map(|xo| (-1..=1).map(move |yo| (xo, yo)))
        .flatten()
        .filter(|&(xo, yo)| xo != 0 || yo != 0)
        .filter_map(|(xo, yo)| Some((r.0.checked_add_signed(xo)?, r.1.checked_add_signed(yo)?)))
        .filter(|p| all.contains(p))
}

fn parse_input(i: &str) -> HashSet<Pos> {
    let mut s = HashSet::new();

    for (l, line) in i.lines().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char == '@' {
                s.insert((l, c));
            }
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = concat!(
        "..@@.@@@@.\n",
        "@@@.@.@.@@\n",
        "@@@@@.@.@@\n",
        "@.@@@@..@.\n",
        "@@.@@@@.@@\n",
        ".@@@@@@@.@\n",
        ".@.@.@.@@@\n",
        "@.@@@.@@@@\n",
        ".@@@@@@@@.\n",
        "@.@.@@@.@.\n",
    );

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_input(concat!("..@..@@\n", "@@..@@.\n", ".@.@.@.\n",)),
            HashSet::from([
                (0, 2),
                (0, 5),
                (0, 6),
                (1, 0),
                (1, 1),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (2, 5),
            ])
        );
    }

    #[test]
    fn test_part_1() {
        let i = parse_input(INPUT);
        assert_eq!(part_1(&i), 13);
    }
}
