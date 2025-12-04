use aoc_derive::aoc_main;
use derive_more::{Deref, DerefMut};
use itertools::Itertools;
use utils::{grid::Grid, math::Vec2D, *};

#[derive(Deref, DerefMut)]
struct Map(Grid<char>);

impl Map {
    fn accessible_positions(&self) -> impl Iterator<Item = Vec2D> {
        self.iter().filter_map(|(pos, val)| {
            (val == &'@'
                && self.all_neighbor_values(&pos).filter(|&neighbor| neighbor == &'@').count() < 4)
                .then_some(pos)
        })
    }

    fn clear_paper(&mut self) {
        for pos in self.accessible_positions().collect_vec() {
            *self.get_mut(pos).unwrap() = '.'
        }
    }

    fn clear_all(&mut self) -> usize {
        let mut count = 0;
        loop {
            let to_clear = self.accessible_positions().count();
            if to_clear == 0 {
                return count;
            }
            count += to_clear;
            self.clear_paper();
        }
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let mut map = Map(input.char_grid());

    (map.accessible_positions().count(), map.clear_all())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#,
            13,
            43
        );
    }
}
