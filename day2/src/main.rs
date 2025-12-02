use aoc_derive::aoc_main;
use itertools::Itertools;
use num::Integer;
use utils::*;

fn invalid(id: usize) -> bool {
    let s = id.to_string();
    s.len().is_even() && s[..s.len() / 2] == s[s.len() / 2..]
}

fn invalid2(id: usize) -> bool {
    let chars = id.to_string().chars().collect_vec();
    (1..=(chars.len() / 2))
        .filter(|&count| chars.len().is_multiple_of(count))
        .any(|count| chars.iter().skip(count).enumerate().all(|(i, &c)| chars[i % count] == c))
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let ranges = input.as_str().trim().split(',').map(|s| {
        let (start, end) = s.split_once('-').unwrap();
        start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
    });

    (
        ranges.clone().flat_map(|r| r.filter(|&i| invalid(i))).sum_usize(),
        ranges.flat_map(|r| r.filter(|&i| invalid2(i))).sum_usize(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#,
            1227775554,
            4174379265_usize
        );

        assert!(!invalid2(10));
        assert!(invalid2(11));
        assert!(invalid2(1111));
        assert!(!invalid2(11110));
        assert!(invalid2(10101010));
        assert!(invalid2(123123));
        assert!(!invalid2(1231231));
        assert!(!invalid2(12312));
        assert!(!invalid2(1231));
        assert!(!invalid2(2));
        assert!(!invalid2(1230123));
    }
}
