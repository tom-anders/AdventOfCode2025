use aoc_derive::aoc_main;
use utils::*;

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let (_, password, password2) =
        input.lines().fold((50, 0, 0), |(dial, password, password2), line| {
            let inc = if line.starts_with('L') { -1 } else { 1 };
            let steps: i64 = line[1..].parse().unwrap();

            let (dial, password2) =
                (0..steps).fold((dial, password2), |(dial, password2), _| match dial + inc {
                    0 => (0, password2 + 1),
                    100 => (0, password2 + 1),
                    -1 => (99, password2),
                    next => (next, password2),
                });
            let password = if dial == 0 { password + 1 } else { password };

            (dial, password, password2)
        });
    (password, password2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_part2!("R1000", 10);
        assert_part2!("L1000", 10);
        assert_part2!("R50", 1);
        assert_part2!("L50", 1);

        assert_example!(
            r#"R49
R1"#,
            1
        );

        assert_example!(
            r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#,
            3,
            6
        );
    }
}
