use std::collections::HashMap;

use aoc_derive::aoc_main;
use utils::{OneOrTwo::*, grid::Grid, *};

#[derive(Debug, Clone)]
struct Beams {
    x: HashMap<i64, usize>,
    y: i64,
}

impl Beams {
    fn solve(mut self, manifold: &Grid<char>) -> (usize, usize) {
        let mut num_splits = 0;
        while self.y + 1 < manifold.num_rows() as i64 {
            self.x = self
                .x
                .into_iter()
                .flat_map(|(x, count)| {
                    if manifold.get((x, self.y + 1)).unwrap() == &'^' {
                        num_splits += 1;
                        Two((x + 1, count), (x - 1, count))
                    } else {
                        One((x, count))
                    }
                })
                .fold(HashMap::new(), |mut map, (x, count)| {
                    *map.entry(x).or_insert(0) += count;
                    map
                });
            self.y += 1;
        }
        (num_splits, self.x.values().sum())
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let manifold = input.char_grid();
    let start = manifold.find_position(&'S').unwrap();

    Beams { x: HashMap::from([(start.x, 1)]), y: start.y }.solve(&manifold)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#,
            21,
            40
        );
    }
}
