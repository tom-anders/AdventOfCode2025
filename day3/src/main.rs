use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

fn find_best(numbers: &[usize], cur: usize, depth: u32, max: usize) -> Option<usize> {
    if depth == 12 {
        return Some(cur);
    }

    if numbers.is_empty() || cur + 10_usize.pow(12 - depth) < max {
        return None;
    }

    let skip_result = find_best(&numbers[1..], cur, depth, max);

    let no_skip_result = find_best(
        &numbers[1..],
        cur + 10_usize.pow(12 - depth - 1) * numbers[0],
        depth + 1,
        skip_result.unwrap_or(max),
    );

    skip_result.max(no_skip_result)
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let banks = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect_vec())
        .collect_vec();

    let part1 = banks
        .iter()
        .map(|bank| {
            bank.iter()
                .enumerate()
                .flat_map(|(i, n1)| bank.iter().skip(i + 1).map(move |n2| 10 * n1 + n2))
                .max()
                .unwrap()
        })
        .sum_usize();

    let part2 = banks.iter().map(|bank| find_best(bank, 0, 0, 0).unwrap()).sum_usize();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"987654321111111
811111111111119
234234234234278
818181911112111"#,
            357,
            3121910778619_usize
        );
    }
}
