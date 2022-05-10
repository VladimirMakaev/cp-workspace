//{"name":"A. Иерархия","group":"Codeforces - CP3 DSU Mashup","url":"https://codeforces.com/group/kgiEExP5XR/contest/380181/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n7 2 3 1\n4\n1 2 5\n2 4 1\n3 4 1\n1 3 5\n","output":"11\n"},{"input":"3\n1 2 3\n2\n3 1 2\n3 1 3\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AIerarkhiya"}}}

use std::collections::HashSet;

use algo_lib::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

#[derive(Clone)]
struct Edge {
    from: usize,
    to: usize,
    weight: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let q: Vec<usize> = input.read_vec(n);
    let m: usize = input.read();
    let mut data: Vec<_> = input
        .read_vec(m)
        .into_iter()
        .map(|(from, to, weight)| Edge { from, to, weight })
        .collect();

    data.sort_by(|x, y| x.weight.cmp(&y.weight));

    let mut set = DSU::new(n + 1);
    let mut has_parent = HashSet::<usize>::new();
    let mut cost = 0;
    for edge in data.iter() {
        if set.find(edge.from) != set.find(edge.to) && !has_parent.contains(&edge.to) {
            set.union(edge.from, edge.to);
            has_parent.insert(edge.to);
            cost += edge.weight;
        }
    }
    if has_parent.len() == n - 1 {
        out!(cost)
    } else {
        out!(-1)
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
