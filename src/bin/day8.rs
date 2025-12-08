use std::{
    collections::HashMap,
    fs::read_to_string,
    sync::atomic::{AtomicUsize, Ordering},
};

use itertools::Itertools;
use winnow::{
    Parser, Result,
    ascii::{dec_uint, line_ending},
    combinator::{repeat, terminated},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CircuitId(usize);

fn main() {
    let i = read_to_string("input/day8").unwrap();
    let i = parse.parse(&i).unwrap();

    println!("part 1: {}", part_1(&i, 1000));
    println!("part 2: {}", part_2(&i));
}

fn part_1(i: &[Pos], limit: usize) -> usize {
    let mut circuits = HashMap::new();

    closest_pairs(i).take(limit).for_each(|(p1, p2)| {
        connect(p1, p2, &mut circuits);
    });

    circuits
        .iter()
        .sorted_by_key(|(_, id)| *id)
        .chunk_by(|(_, id)| *id)
        .into_iter()
        .map(|a| a.1.count())
        .sorted_by(|x1, x2| x1.cmp(x2).reverse())
        .take(3)
        .product()
}

fn part_2(i: &[Pos]) -> u64 {
    let mut circuits = i.iter().map(|p| (*p, CircuitId::new())).collect();

    let last_conn = closest_pairs(i)
        .take_while_inclusive(|(p1, p2)| {
            connect(p1, p2, &mut circuits);
            !circuits.values().all_equal()
        })
        .last()
        .unwrap();

    last_conn.0.x * last_conn.1.x
}

fn closest_pairs(i: &[Pos]) -> impl Iterator<Item = (&Pos, &Pos)> {
    assert!(i.iter().all_unique());

    i.iter()
        .tuple_combinations()
        .sorted_by(|(p1, p2), (p3, p4)| dist(p1, p2).cmp(&dist(p3, p4)))
}

fn dist(p1: &Pos, p2: &Pos) -> u64 {
    p1.x.abs_diff(p2.x).pow(2) + p1.y.abs_diff(p2.y).pow(2) + p1.z.abs_diff(p2.z).pow(2)
}

fn connect(p1: &Pos, p2: &Pos, circuits: &mut HashMap<Pos, CircuitId>) {
    let g1 = circuits.get(p1).cloned();
    let g2 = circuits.get(p2).cloned();
    match (g1, g2) {
        (None, None) => {
            let id = CircuitId::new();
            circuits.insert(*p1, id.clone());
            circuits.insert(*p2, id);
        }
        (None, Some(id2)) => {
            circuits.insert(*p1, id2.clone());
        }
        (Some(id1), None) => {
            circuits.insert(*p2, id1.clone());
        }
        (Some(id1), Some(id2)) if id1 == id2 => {}
        (Some(id1), Some(id2)) => {
            let rem = circuits.extract_if(|_, id| *id == id2).collect::<Vec<_>>();
            circuits.extend(rem.iter().map(|(p, _)| (*p, id1.clone())));
        }
    }
}

fn parse(i: &mut &str) -> Result<Vec<Pos>> {
    repeat(
        1..,
        terminated(
            (
                dec_uint::<_, u64, _>,
                ',',
                dec_uint::<_, u64, _>,
                ',',
                dec_uint::<_, u64, _>,
            )
                .map(|(x, _, y, _, z)| Pos { x, y, z }),
            line_ending,
        ),
    )
    .parse_next(i)
}

impl CircuitId {
    pub fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = concat!(
        "162,817,812\n",
        "57,618,57\n",
        "906,360,560\n",
        "592,479,940\n",
        "352,342,300\n",
        "466,668,158\n",
        "542,29,236\n",
        "431,825,988\n",
        "739,650,466\n",
        "52,470,668\n",
        "216,146,977\n",
        "819,987,18\n",
        "117,168,530\n",
        "805,96,715\n",
        "346,949,466\n",
        "970,615,88\n",
        "941,993,340\n",
        "862,61,35\n",
        "984,92,344\n",
        "425,690,689\n",
    );

    #[test]
    fn test_parse() {
        assert_eq!(
            parse.parse("1,2,3\n4,5,6\n"),
            Ok(vec![Pos { x: 1, y: 2, z: 3 }, Pos { x: 4, y: 5, z: 6 },])
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse.parse(INPUT).unwrap(), 10), 40);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse.parse(INPUT).unwrap()), 25272);
    }
}
