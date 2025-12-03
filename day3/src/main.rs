use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

fn find_joltage(bank: &[usize], num_digits: usize) -> usize {
    bank.iter()
        .enumerate()
        .fold(Vec::with_capacity(num_digits), |mut stack, (pos, &battery)| {
            let remaining = bank.len() - pos - 1;

            while let Some(&back) = stack.last()
                && battery > back
                && stack.len() + remaining >= num_digits
            {
                stack.pop();
            }
            if stack.len() < num_digits {
                stack.push(battery);
            }
            stack
        })
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, n)| acc + n * 10_usize.pow(i as u32))
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let banks = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect_vec());

    (
        banks.clone().map(|bank| find_joltage(&bank, 2)).sum_usize(),
        banks.map(|bank| find_joltage(&bank, 12)).sum_usize(),
    )
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
