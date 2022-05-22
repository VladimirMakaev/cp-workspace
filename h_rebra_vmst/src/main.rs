//{"name":"H. Ребра в MST","group":"Codeforces - CP3 DSU Mashup","url":"https://codeforces.com/group/kgiEExP5XR/contest/380181/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5\n1 2 101\n1 3 100\n2 3 2\n2 4 2\n3 4 1\n","output":"none\nany\nat least one\nat least one\nany\n"},{"input":"3 3\n1 2 1\n2 3 1\n1 3 2\n","output":"any\nany\nnone\n"},{"input":"3 3\n1 2 1\n2 3 1\n1 3 1\n","output":"at least one\nat least one\nat least one\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HRebraVMST"}}}

use std::collections::BinaryHeap;

use algo_lib::dsu_r::DSUR;
use algo_lib::graph::{self, Graph};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

const NONE: u8 = 1;
const ANY: u8 = 2;
const AT_LEAST_ONE: u8 = 3;

#[derive(Debug, PartialEq, Eq, Ord)]
struct Edge {
    index: usize,
    from: usize,
    to: usize,
    weight: usize,
}

impl graph::Edge for Edge {
    fn get_from(&self) -> usize {
        self.from - 1
    }

    fn get_to(&self) -> usize {
        self.to - 1
    }

    fn get_id(&self) -> usize {
        self.index
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let res = other.weight.partial_cmp(&self.weight);
        match res {
            Some(std::cmp::Ordering::Equal) => other.index.partial_cmp(&self.index),
            _ => res,
        }
    }
}

fn pop_next_size(edges: &mut BinaryHeap<Edge>) -> Option<Vec<Edge>> {
    if let Some(next) = edges.pop() {
        let mut batch = Vec::new();
        let w = next.weight;
        batch.push(next);

        while let Some(next) = edges.peek() {
            if next.weight == w {
                batch.push(edges.pop().unwrap());
            } else {
                break;
            }
        }

        return Some(batch);
    }
    None
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = input.read::<(usize, usize)>();
    //We store edges in min-heap
    let mut edges = input
        .read_vec::<(usize, usize, usize)>(m)
        .into_iter()
        .enumerate()
        .map(|(index, x)| Edge {
            index,
            from: x.0,
            to: x.1,
            weight: x.2,
        })
        .collect::<BinaryHeap<Edge>>();

    let mut result = vec![NONE; m];
    let mut set = DSUR::new(n + 1);

    let mut g = Graph::new(n);

    while let Some(next_size) = pop_next_size(&mut edges) {
        for edge in next_size.iter() {
            if set.find(edge.from) != set.find(edge.to) {
                result[edge.index] = AT_LEAST_ONE;
            }
        }
        for edge in next_size.iter() {
            set.union(edge.from, edge.to);
            g.add_unidirected_edge(edge);
        }

        for edge_index in g.build_low_link().bridges() {
            result[*edge_index] = ANY;
        }
    }

    for i in 0..result.len() {
        out_line!(match result[i] {
            NONE => "none",
            ANY => "any",
            AT_LEAST_ONE => "at least one",
            _ => panic!("unreachable"),
        });
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
