use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

type Pos = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
struct Input {
    start: Pos,
    splitters: Vec<Pos>,
}

fn main() {
    let i = read_to_string("input/day7").unwrap();
    let i = parse(&i);

    println!("part 1: {}", part_1(&i));
    println!("part 2: {}", part_2(&i));
}

fn part_1(i: &Input) -> u64 {
    let mut handled: HashSet<Pos> = HashSet::new();

    count_splits(&i.start, &i.splitters, &mut handled)
}

fn part_2(i: &Input) -> u64 {
    let mut cache = HashMap::new();
    count_timelines(&i.start, &i.splitters, &mut cache) + 1
}

fn count_splits(start: &Pos, splitters: &[Pos], handled: &mut HashSet<Pos>) -> u64 {
    match splitters
        .iter()
        .filter(|&p| p.0 > start.0 && p.1 == start.1)
        .sorted_by_key(|p| p.0)
        .next()
    {
        Some(s) if !handled.contains(s) => {
            handled.insert(*s);
            1 + count_splits(&(s.0, s.1 - 1), splitters, handled)
                + count_splits(&(s.0, s.1 + 1), splitters, handled)
        }
        Some(_) => 0,
        None => 0,
    }
}

fn count_timelines(start: &Pos, splitters: &[Pos], cache: &mut HashMap<Pos, u64>) -> u64 {
    if cache.contains_key(start) {
        return *cache.get(start).unwrap();
    }

    let c = match splitters
        .iter()
        .filter(|&p| p.0 > start.0 && p.1 == start.1)
        .sorted_by_key(|p| p.0)
        .next()
    {
        Some(s) => {
            1 + count_timelines(&(s.0, s.1 - 1), splitters, cache)
                + count_timelines(&(s.0, s.1 + 1), splitters, cache)
        }
        None => 0,
    };

    cache.insert(*start, c);

    c
}

fn parse(i: &str) -> Input {
    let mut start = None;
    let mut splitters = vec![];

    for (l, line) in i.lines().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char == 'S' {
                start = Some((l, c));
            }
            if char == '^' {
                splitters.push((l, c));
            }
        }
    }

    Input {
        start: start.unwrap(),
        splitters,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = concat!(
        ".......S.......\n",
        "...............\n",
        ".......^.......\n",
        "...............\n",
        "......^.^......\n",
        "...............\n",
        ".....^.^.^.....\n",
        "...............\n",
        "....^.^...^....\n",
        "...............\n",
        "...^.^...^.^...\n",
        "...............\n",
        "..^...^.....^..\n",
        "...............\n",
        ".^.^.^.^.^...^.\n",
        "...............\n",
    );

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(INPUT),
            Input {
                start: (0, 7),
                splitters: vec![
                    (2, 7),
                    (4, 6),
                    (4, 8),
                    (6, 5),
                    (6, 7),
                    (6, 9),
                    (8, 4),
                    (8, 6),
                    (8, 10),
                    (10, 3),
                    (10, 5),
                    (10, 9),
                    (10, 11),
                    (12, 2),
                    (12, 6,),
                    (12, 12),
                    (14, 1),
                    (14, 3),
                    (14, 5),
                    (14, 7),
                    (14, 9),
                    (14, 13)
                ]
            }
        )
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse(INPUT)), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse(INPUT)), 40);
    }
}
