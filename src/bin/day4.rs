use std::{collections::HashSet, fs};

type Pos = (usize, usize);

fn main() {
    let i = fs::read_to_string("input/day4").unwrap();
    let i = parse_input(&i);

    println!("part 1: {}", part_1(&i));
    println!("part 2: {}", part_2(&i));
}

fn part_1(rolls: &HashSet<Pos>) -> usize {
    rolls
        .iter()
        .filter(|r| adjacent(r, rolls).count() < 4)
        .count()
}

fn part_2(rolls: &HashSet<Pos>) -> usize {
    let mut rolls = rolls.clone();
    let mut to_check = rolls.clone();
    let mut sum = 0;

    loop {
        let c = rolls.clone();
        let removed = to_check
            .iter()
            .map(|r| (*r, adjacent(r, &c).collect::<Vec<_>>()))
            .filter(|(_, a)| a.len() < 4)
            .inspect(|(r, _)| {
                rolls.remove(r);
            })
            .collect::<Vec<_>>();

        sum += removed.len();

        if removed.is_empty() {
            break;
        }

        to_check = removed
            .iter()
            .flat_map(|r| r.1.iter())
            .copied()
            .collect();
        removed.iter().for_each(|(r, _)| {
            to_check.remove(r);
        });
    }

    sum
}

fn adjacent(r: &Pos, all: &HashSet<Pos>) -> impl Iterator<Item = Pos> + Clone {
    (-1..=1)
        .flat_map(|xo| (-1..=1).map(move |yo| (xo, yo)))
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

    #[test]
    fn test_part_2() {
        let i = parse_input(INPUT);
        assert_eq!(part_2(&i), 43);
    }
}
