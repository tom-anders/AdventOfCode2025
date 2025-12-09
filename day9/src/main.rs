use std::ops::RangeInclusive;

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::math::Vec2D;
use utils::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rect {
    range_x: RangeInclusive<i64>,
    range_y: RangeInclusive<i64>,
}

impl Rect {
    fn new(a: &Vec2D, b: &Vec2D) -> Self {
        Self { range_x: a.x.min(b.x)..=a.x.max(b.x), range_y: a.y.min(b.y)..=a.y.max(b.y) }
    }

    fn area(&self) -> i64 {
        (self.range_x.try_len().unwrap() * self.range_y.try_len().unwrap()) as i64
    }

    fn intersects(&self, (v1, v2): &(Vec2D, Vec2D)) -> bool {
        let vx = v1.x.min(v2.x)..=v1.x.max(v2.x);
        let vy = v1.y.min(v2.y)..=v1.y.max(v2.y);

        if vx.end() <= self.range_x.start() || vx.start() >= self.range_x.end() {
            return false;
        }

        if vy.end() <= self.range_y.start() || vy.start() >= self.range_y.end() {
            return false;
        }

        true
    }
}

fn part2(poly: &[Vec2D]) -> i64 {
    let vertices: Vec<(Vec2D, Vec2D)> = poly.iter().copied().circular_tuple_windows().collect();

    poly.iter()
        .tuple_combinations()
        .map(|(a, b)| Rect::new(a, b))
        .filter(|rect| vertices.iter().all(|v| !rect.intersects(v)))
        .map(|r| r.area())
        .max()
        .unwrap()
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let poly = input
        .lines()
        .map(|line| {
            let (x, y) = extract_numbers(line).collect_tuple().unwrap();
            Vec2D::new(x, y)
        })
        .collect_vec();

    (poly.iter().tuple_combinations().map(|(a, b)| Rect::new(a, b).area()).max(), part2(&poly))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#,
            50,
            24
        );
    }
}
