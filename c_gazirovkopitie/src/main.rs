//{"name":"C. Газировкопитие","group":"Codeforces - CP3 DSU Mashup","url":"https://codeforces.com/group/kgiEExP5XR/contest/380181/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4 5 10 8 100 3 1\n","output":"2\n"},{"input":"5 100 10 1 19 90 4 3\n","output":"3\n"},{"input":"10 1000 1000 25 23 1 50 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CGazirovkopitie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    
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
