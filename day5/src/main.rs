use std::ops::RangeInclusive;

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

fn part2(mut ranges: Vec<RangeInclusive<usize>>) -> usize {
    ranges.sort_by_key(|r| *r.start());
    loop {
        let prev = ranges.len();
        let mut i = 0;
        while i < ranges.len() - 1 {
            if ranges[i].end() >= ranges[i + 1].start() {
                ranges[i] = *ranges[i].start()..=(*ranges[i + 1].end().max(ranges[i].end()));
                ranges.remove(i + 1);
            } else {
                i += 1;
            }
        }

        if ranges.len() == prev {
            return ranges.iter().map(|range| range.try_len().unwrap()).sum();
        }
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let (ranges, ingredients) = input.raw.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split('-').collect_tuple().unwrap();
            start.parse_usize()..=end.parse_usize()
        })
        .collect_vec();
    let ingredients = ingredients.lines().map(|line| line.parse::<usize>().unwrap()).collect_vec();

    (ingredients.iter().filter(|i| ranges.iter().any(|r| r.contains(i))).count(), part2(ranges))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#,
            3,
            14
        );
    }
}
