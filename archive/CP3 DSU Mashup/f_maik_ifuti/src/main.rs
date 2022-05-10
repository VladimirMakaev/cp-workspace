//{"name":"F. Майк и футы","group":"Codeforces - CP3 DSU Mashup","url":"https://codeforces.com/group/kgiEExP5XR/contest/380181/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1 2 3 4 5 4 3 2 1 6\n","output":"6 4 4 3 3 2 2 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMaikIFuti"}}}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve_dp(data: &Vec<usize>) -> Vec<usize> {
    let n = data.len();

    let mut result = Vec::with_capacity(n);

    let mut dp = vec![data.clone(), vec![0; n]];
    result.push(dp[0].iter().max().cloned().unwrap());
    for i in 1..n {
        for j in i..n {
            dp[i & 1][j] = std::cmp::min(dp[(i - 1) & 1][j - 1], data[j])
        }

        result.push(dp[i & 1][i..n].iter().max().cloned().unwrap());
    }
    return result;
}

fn solve_heap(data: &Vec<usize>) -> Vec<usize> {
    let mut queue: BinaryHeap<(usize, usize, usize)> = data
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, x)| (x, i, 0))
        .collect();

    let mut result = Vec::with_capacity(data.len());
    let mut cur_level = 0;
    while let Some((x, i, level)) = queue.pop() {
        let x = x;
        let i = i;
        let level = level;
        if cur_level == level {
            result.push(x);
            cur_level += 1;
        }
        if i < data.len() - 1 {
            let next = std::cmp::min(x, data[i + 1]);
            queue.push((next, i + 1, level + 1));
        }

        if cur_level == data.len() {
            break;
        }
    }
    return result;
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let data = input.read_vec::<usize>(n);
    out!(solve_heap(&data));

    // let queue: BinaryHeap<(Reverse<usize>, usize, usize)> = data
    //     .into_iter()
    //     .enumerate()
    //     .map(|(x, i)| (Reverse(x), i, 0))
    //     .collect();

    // let mut result: Vec<usize> = Vec::with_capacity(n);
    // let cur_level = 0;
    // while let Some((Reverse(x), i, level)) = queue.pop() {
    //     if cur_level == level {
    //         result.push(x);
    //     }

    //     if level == n - 1 {
    //         break;
    //     }
    // }
    // out!(result);
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
