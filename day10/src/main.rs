use std::str::FromStr;

use z3::ast::Int;
use z3::*;

use aoc_derive::aoc_main;
use itertools::Itertools;
use lazy_regex::regex;
use utils::{
    graphs::{UnweightedGraph, bfs},
    *,
};

#[derive(Debug, Clone)]
struct Machine {
    target_bitmask: usize,
    buttons: Vec<Vec<usize>>,
    target_joltages: Vec<usize>,
}

impl Machine {
    fn solve(&self) -> usize {
        bfs(self, 0_usize, self.target_bitmask).distance.unwrap()
    }

    fn solve_part2(&self) -> usize {
        let matrix = (0..self.target_joltages.len())
            .map(|idx| {
                self.buttons
                    .iter()
                    .map(|buttons| if buttons.contains(&idx) { 1 } else { 0 })
                    .collect_vec()
            })
            .collect_vec();

        let opt = Optimize::new();

        let num_presses =
            (0..self.buttons.len()).map(|i| Int::new_const(format!("x_{}", i))).collect_vec();

        for x in &num_presses {
            opt.assert(&x.ge(Int::from_i64(0)));
        }

        // Hand-written Matrix-vector product: matrix * x = target_joltages
        for (row, target_joltage) in matrix.iter().zip(self.target_joltages.iter()) {
            let sum = row
                .iter()
                .zip(num_presses.iter())
                .map(|(&presses, target_joltage)| presses * target_joltage)
                .sum::<Int>();
            opt.assert(&sum.eq(Int::from(*target_joltage as i64)));
        }

        opt.minimize(&num_presses.iter().sum::<Int>());

        // Solve
        match opt.check(&[]) {
            SatResult::Sat => {
                let model = opt.get_model().unwrap();
                num_presses
                    .iter()
                    .map(|var| model.eval(var, true).unwrap().as_i64().unwrap())
                    .sum_usize()
            }
            _ => panic!(),
        }
    }
}

impl UnweightedGraph for Machine {
    type Node = usize;

    fn neighbors<'a, 'b: 'a>(
        &'a self,
        node: &'b Self::Node,
    ) -> impl Iterator<Item = Self::Node> + 'a {
        self.buttons
            .iter()
            .map(|buttons| buttons.iter().fold(*node, |next, &idx| next ^ (1 << idx)))
    }
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target, buttons, joltages) = regex!(r#"\[(.*)\] (.*) \{(.*)\}"#).capture_into_tuple(s);

        let target_bitmask = target
            .chars()
            .enumerate()
            .fold(0, |val, (i, c)| if c == '#' { val ^ (1 << i) } else { val });

        let buttons =
            buttons.split_whitespace().map(|s| extract_numbers(s).collect_vec()).collect_vec();
        let target_joltages = extract_numbers(joltages).collect_vec();

        Ok(Self { target_bitmask, buttons, target_joltages })
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let machines = input.lines().flat_map(Machine::from_str);

    (machines.clone().map(|m| m.solve()).sum_usize(), machines.map(|m| m.solve_part2()).sum_usize())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#,
            7,
            33
        );
    }
}
