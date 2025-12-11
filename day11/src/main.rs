use std::collections::HashMap;

use aoc_derive::aoc_main;
use itertools::Itertools;
use utils::*;

#[derive(Clone, Debug)]
struct Node<'a> {
    label: &'a str,
    next: Vec<&'a str>,
}

impl<'a> Node<'a> {
    fn find_out(&'a self, graph: &HashMap<&'a str, Node>) -> usize {
        self.next.iter().map(move |&n| if n == "out" { 1 } else { graph[n].find_out(graph) }).sum()
    }

    fn find_out2(
        &'a self,
        graph: &'a HashMap<&'a str, Node>,
        dac: bool,
        fft: bool,
        cache: &mut HashMap<(&'a str, bool, bool), usize>,
    ) -> usize {
        if let Some(cached) = cache.get(&(self.label, dac, fft)) {
            return *cached;
        }
        let res = self
            .next
            .iter()
            .map(|&n| {
                if n == "out" {
                    if dac && fft { 1 } else { 0 }
                } else {
                    graph[n].find_out2(graph, dac || n == "dac", fft || n == "fft", cache)
                }
            })
            .sum();

        cache.insert((self.label, dac, fft), res);

        res
    }
}

#[aoc_main]
fn solve(input: Input) -> impl Into<Solution> {
    let graph: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (label, rhs) = line.split(':').collect_tuple().unwrap();
            (label, Node { label, next: rhs.split_whitespace().collect() })
        })
        .collect();

    let start = graph.iter().find_map(|(&label, node)| (label == "you").then_some(node));
    let server = graph.iter().find_map(|(&label, node)| (label == "svr").then_some(node));
    (
        start.map(|start| start.find_out(&graph)).unwrap_or_default(),
        server
            .map(|server| server.find_out2(&graph, false, false, &mut HashMap::new()))
            .unwrap_or_default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        use utils::assert_example;
        assert_example!(
            r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#,
            5
        );

        assert_part2!(
            r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#,
            2
        );
    }
}
