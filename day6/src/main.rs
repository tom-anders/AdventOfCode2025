use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    (
        input
            .str_grid()
            .cols()
            .map(|col| {
                let mut col = col.rev();
                let (_, op) = col.next().unwrap();
                col.map(|(_, n)| n.parse_usize())
                    .reduce(|acc, n| if *op == "+" { acc + n } else { acc * n })
                    .unwrap()
            })
            .sum_usize(),
        input
            .char_grid()
            .cols()
            .map(|col| col.values().copied().collect_vec())
            .chunk_by(|col| col.iter().all(|&c| c == ' '))
            .into_iter()
            .filter_map(|(is_whitespace, cols)| (!is_whitespace).then_some(cols.collect_vec()))
            .map(|cols| {
                let op = *cols.first().unwrap().last().unwrap();
                cols.into_iter()
                    .map(|col| col.iter().copied().fold_chars_to_number() as usize)
                    .reduce(|acc, n| if op == '+' { acc + n } else { acc * n })
                    .unwrap()
            })
            .sum_usize(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        pretty_assertions::assert_eq!(
            solve(Input::from_str_no_trim(
                r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#,
            ))
            .into(),
            (4277556, 3263827).into(),
        );
    }
}
