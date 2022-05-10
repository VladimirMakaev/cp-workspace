//{"name":"B. Коровоконг объединяет нацию","group":"Codeforces - CP3 DSU Mashup","url":"https://codeforces.com/group/kgiEExP5XR/contest/380181/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4 1 2\n1 3\n1 2\n","output":"2\n"},{"input":"3 3 1\n2\n1 2\n1 3\n2 3\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BKorovokongObedinyaetNatsiyu"}}}

use std::collections::{HashMap, HashSet};

use algo_lib::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m, k) = input.read::<(usize, usize, usize)>();
    let capitals: HashSet<usize> = input.read_vec(k).into_iter().collect();
    let edges = input.read_vec::<(usize, usize)>(m);
    let mut dsu = DSU::new(n + 1);

    // instead of adjacency matrix i use hashset to mark vertices connected
    let mut linked_set = HashSet::<(usize, usize)>::new();

    // building initial dsu's of connected pieces
    for (from, to) in edges.into_iter() {
        linked_set.insert((from, to));
        linked_set.insert((to, from));
        dsu.union(from, to);
    }

    // mapping from a set to a capital
    let set_to_capital: HashMap<usize, usize> =
        capitals.iter().map(|c| (dsu.find(*c), *c)).collect();

    let mut result: usize = 0;

    let mut no_path_to_cap = Vec::new();

    // finding all vertices not connected to capitals
    for i in 1..=n {
        if !set_to_capital.contains_key(&dsu.find(i)) {
            no_path_to_cap.push(i);
        }
    }

    // capital connected to a max sized connected graph
    let max_size_capital = capitals
        .iter()
        .max_by(|l, r| dsu.find_with_size(**l).1.cmp(&dsu.find_with_size(**r).1))
        .cloned()
        .unwrap();

    //fill nodes not connected to any capital to a full graph
    for i in 0..no_path_to_cap.len() {
        for j in i + 1..no_path_to_cap.len() {
            if !linked_set.contains(&(no_path_to_cap[i], no_path_to_cap[j])) {
                dsu.union(no_path_to_cap[i], no_path_to_cap[j]);
                linked_set.insert((no_path_to_cap[i], no_path_to_cap[j]));
                linked_set.insert((no_path_to_cap[j], no_path_to_cap[i]));
                result += 1;
            }
        }
    }

    //connect any vertex (first one) to a capital of max size
    if !no_path_to_cap.is_empty() {
        dsu.union(no_path_to_cap[0], max_size_capital);
        linked_set.insert((no_path_to_cap[0], max_size_capital));
        linked_set.insert((max_size_capital, no_path_to_cap[0]));
        result += 1;
    }

    // fill the rest to a full graph
    for i in 1..=n {
        for j in i + 1..=n {
            if dsu.find(i) == dsu.find(j) && !linked_set.contains(&(i, j)) {
                result += 1;
                linked_set.insert((i, j));
                linked_set.insert((j, i));
                dsu.union(i, j);
            }
        }
    }

    out!(result);
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
