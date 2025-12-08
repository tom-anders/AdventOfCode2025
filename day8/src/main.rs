use std::collections::{BTreeSet, HashMap, HashSet};

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn dist(&self, other: &Pos) -> i64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
            .floor() as i64
    }
}

#[derive(Debug, Clone)]
struct JunctionBox {
    pos: Pos,
    connections: HashSet<Pos>,
}

impl JunctionBox {
    fn get_circuit(&self, boxes: &HashMap<Pos, JunctionBox>) -> HashSet<Pos> {
        let mut circuit = HashSet::from([self.pos]);
        let mut next = self.connections.clone();
        while !next.is_empty() {
            circuit.extend(next.iter().copied());
            next = next
                .iter()
                .flat_map(|b| boxes[b].connections.iter().copied())
                .filter(|b| !circuit.contains(b))
                .collect();
        }
        circuit
    }
}

fn connect_next(
    distances: &[(Pos, Pos)],
    boxes: &mut HashMap<Pos, JunctionBox>,
) -> Option<(Pos, Pos)> {
    let pair = distances.iter().find(|(b1, b2)| !boxes[b1].connections.contains(b2));
    if let Some((shortest1, shortest2)) = pair {
        boxes.get_mut(shortest1).unwrap().connections.insert(*shortest2);
        boxes.get_mut(shortest2).unwrap().connections.insert(*shortest1);
    }

    pair.copied()
}

fn part1(
    distances: &[(Pos, Pos)],
    mut boxes: HashMap<Pos, JunctionBox>,
    iterations: usize,
) -> usize {
    for _ in 0..iterations {
        connect_next(distances, &mut boxes);
    }

    boxes
        .values()
        .map(|b| b.get_circuit(&boxes).into_iter().collect::<BTreeSet<_>>())
        .unique()
        .map(|c| c.len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>()
}

fn part2(distances: &[(Pos, Pos)], mut boxes: HashMap<Pos, JunctionBox>) -> i64 {
    loop {
        let (b1, b2) = connect_next(distances, &mut boxes)
            .expect("Should find solution before running out of connections");
        if boxes.get(&b1).unwrap().get_circuit(&boxes).len() == boxes.len() {
            return b1.x * b2.x;
        }
    }
}

#[aoc_main(1000)]
fn solve(input: Input, iterations: usize) -> impl Into<Solution> {
    let boxes: HashMap<_, JunctionBox> = input
        .lines()
        .map(|line| {
            let (x, y, z) = extract_numbers(line).collect_tuple().unwrap();
            let pos = Pos { x, y, z };
            (pos, JunctionBox { pos, connections: HashSet::new() })
        })
        .collect();

    let distances = boxes
        .values()
        .enumerate()
        .flat_map(|(i, b1)| {
            boxes
                .values()
                .enumerate()
                .filter_map(move |(j, b2)| (j < i).then_some((b1.pos, b2.pos)))
        })
        .sorted_by_key(|(b1, b2)| b1.dist(b2))
        .collect_vec();

    (part1(&distances, boxes.clone(), iterations), part2(&distances, boxes))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        pretty_assertions::assert_eq!(
            solve(
                Input::from(
                    r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#
                ),
                10
            )
            .into(),
            Solution::from((40.to_string(), 25272.to_string()))
        );
    }
}
