//{"name":"E. Блоха","group":"Codeforces - CP3 DSU Mashup","url":"https://codeforces.com/group/kgiEExP5XR/contest/380181/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3 1000000\n","output":"6\n"},{"input":"3 3 2\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBlokha"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();
    let s: usize = input.read();

    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(m);

    for i in 0..std::cmp::min(n, s) {
        let left = i / s;
        let right = (n - i - 1) / s;
        a.push(left + right + 1);
    }

    for i in 0..std::cmp::min(m, s) {
        let up = i / s;
        let down = (m - i - 1) / s;
        b.push(up + down + 1);
    }

    let max_a = a.iter().max().cloned().unwrap();
    let max_b = b.iter().max().cloned().unwrap();

    let a: usize = a.into_iter().filter(|x| *x == max_a).sum();
    let b: usize = b.into_iter().filter(|x| *x == max_b).sum();
    out!(a * b);
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
