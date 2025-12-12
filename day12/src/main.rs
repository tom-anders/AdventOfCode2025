use aoc_derive::aoc_main;
use itertools::{Itertools, iproduct};
use lazy_regex::regex;
use utils::RegexHelper;
use utils::grid::Grid;
use utils::math::Vec2D;
use utils::*;

#[derive(Debug, Clone)]
struct Shape {
    display: String,
    rotations: Vec<Vec<Vec2D>>,
}

impl Shape {
    fn new(s: &str) -> Self {
        let base = s.lines().skip(1).map(|line| line.chars().collect_vec()).collect_vec();

        // Both puzzle input and example have 3x3 shapes, so we can hardcode this
        fn rotate(shape: &[Vec<char>]) -> Vec<Vec<char>> {
            vec![
                vec![shape[2][0], shape[1][0], shape[0][0]],
                vec![shape[2][1], shape[1][1], shape[0][1]],
                vec![shape[2][2], shape[1][2], shape[0][2]],
            ]
        }

        let rot90 = rotate(&base);
        let rot180 = rotate(&rot90);
        let rot270 = rotate(&rot180);

        Self {
            display: base.iter().map(|line| line.iter().join("")).join("\n"),
            rotations: [base, rot90, rot180, rot270]
                .into_iter()
                .unique()
                .map(|shape| {
                    shape
                        .iter()
                        .enumerate()
                        .flat_map(|(y, line)| {
                            line.iter().enumerate().filter_map(move |(x, c)| {
                                (*c == '#').then_some(Vec2D::from((x, y)))
                            })
                        })
                        .collect_vec()
                })
                .collect_vec(),
        }
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    shape_req: Vec<usize>,
}

impl Region {
    fn new(s: &str) -> Self {
        let (width, height, shapes) = regex!(r#"(\d+)x(\d+): (.*)"#).capture_into_tuple(s);
        Self {
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
            shape_req: shapes.split_whitespace().map(|n| n.parse().unwrap()).collect_vec(),
        }
    }

    fn solve(&self, shapes: &[Shape]) -> bool {
        self.solve_impl(
            self.shape_req.clone(),
            &mut vec![],
            shapes,
            &mut Grid::new(vec![vec!['.'; self.width]; self.height]),
        )
    }

    fn solve_impl(
        &self,
        shapes_left: Vec<usize>,
        insertions: &mut Vec<(usize, Vec2D)>,
        available_shapes: &[Shape],
        grid: &mut Grid<char>,
    ) -> bool {
        if shapes_left.iter().all(|&n| n == 0) {
            return true;
        }

        if grid.iter().filter(|(_, c)| **c == '.').count()
            < shapes_left
                .iter()
                .enumerate()
                .map(|(i, n)| n * available_shapes[i].display.chars().filter(|&c| c == '#').count())
                .sum()
        {
            return false;
        }

        for (x, y) in iproduct!(0..grid.num_cols() - 2, 0..grid.num_rows() - 2) {
            let top_left = Vec2D::from((x, y));
            for (i, &num_shapes) in shapes_left.iter().enumerate() {
                if num_shapes == 0 {
                    continue;
                }
                for rot in &available_shapes[i].rotations {
                    let fits = rot.iter().all(|&pos| grid[top_left + pos] == '.');
                    if fits {
                        for pos in rot {
                            grid[top_left + *pos] = '#';
                        }

                        let mut new_shapes_left = shapes_left.clone();
                        new_shapes_left[i] -= 1;

                        insertions.push((i, top_left));

                        // println!("Inserted shape {i} @{}", top_left);
                        let solved =
                            self.solve_impl(new_shapes_left, insertions, available_shapes, grid);

                        for pos in rot {
                            grid[top_left + *pos] = '.';
                        }
                        insertions.pop();

                        if solved {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let blocks = input.blocks().collect_vec();
    let (regions, shapes) = blocks.split_last().unwrap();
    let shapes = shapes.iter().copied().map(Shape::new).collect_vec();

    let regions = regions.lines().map(Region::new);

    regions.filter(|r| r.solve(&shapes)).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#,
            2
        );
    }
}
