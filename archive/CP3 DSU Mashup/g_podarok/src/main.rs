//{"name":"G. Подарок","group":"Codeforces - CP3 DSU Mashup","url":"https://codeforces.com/group/kgiEExP5XR/contest/380181/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n2 1\n1 2 10 15\n1 2 4 20\n1 3 5 1\n","output":"30\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GPodarok"}}}

use std::collections::BinaryHeap;

use algo_lib::io::input::{Input, Readable};
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

#[derive(PartialEq, Eq)]
struct Edge {
    from: usize,
    to: usize,
    price_g: usize,
    price_s: usize,
    price_t: usize,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.price_t.cmp(&other.price_t);
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = input.read::<(usize, usize)>();
    let (g, s) = input.read::<(usize, usize)>();
    let edges = input.read_vec::<(usize, usize, usize, usize)>(m);

    let mut heap = BinaryHeap::<Edge>::new();

    for (from, to, price_g, price_s) in edges.iter() {
        heap.push(Edge {
            from: *from,
            to: *to,
            price_g: *price_g,
            price_s: *price_s,
            price_t: g * price_g + s * price_s,
        })
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
