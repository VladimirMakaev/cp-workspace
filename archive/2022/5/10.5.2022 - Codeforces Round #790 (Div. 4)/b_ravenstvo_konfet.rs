//{"name":"B. Равенство конфет","group":"Codeforces - Codeforces Round #790 (Div. 4)","url":"https://codeforces.com/contest/1676/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n1 2 3 4 5\n6\n1000 1000 5 1000 1000 1000\n10\n1 2 3 5 1 2 7 9 13 5\n3\n8 8 8\n1\n10000000\n","output":"10\n4975\n38\n0\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BRavenstvoKonfet"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: i32 = input.read();
    let data: Vec<i32> = input.read_vec(n as usize);
    let min = data.iter().min().cloned().unwrap();
    let max = data.iter().max().cloned().unwrap();
    let sum: i32 = data.iter().map(|x| x - min).sum();
    out_line!(sum);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
