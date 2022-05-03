//{"name":"D. Разноцветная печать","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"12\n5\nBRBBW\n1\nB\n2\nWB\n2\nRW\n3\nBRB\n3\nRBB\n7\nWWWWWWW\n9\nRBWBWRRBW\n10\nBRBRBRBRRB\n12\nBBBRWWRRRWBR\n10\nBRBRBRBRBW\n5\nRBWBW\n","output":"YES\nNO\nNO\nNO\nYES\nYES\nYES\nNO\nYES\nNO\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRaznotsvetnayaPechat"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    
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
