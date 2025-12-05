use std::ops::RangeInclusive;

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

fn part2(ranges: &[RangeInclusive<usize>]) -> usize {
    let ranges = ranges.iter().cloned().sorted_by_key(|r| *r.start()).collect_vec();
    ranges
        .iter()
        // Add an empty range at the end to make sure we add the last one to the counter
        .chain(std::iter::once(&(usize::MAX..=usize::MAX)))
        .fold((0, ranges.first().unwrap().clone()), |(count, curr), next| {
            if next.start() <= curr.end() {
                (count, *curr.start()..=*curr.end().max(next.end()))
            } else {
                (count + curr.try_len().unwrap(), next.clone())
            }
        })
        .0
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

    (ingredients.iter().filter(|i| ranges.iter().any(|r| r.contains(i))).count(), part2(&ranges))
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
