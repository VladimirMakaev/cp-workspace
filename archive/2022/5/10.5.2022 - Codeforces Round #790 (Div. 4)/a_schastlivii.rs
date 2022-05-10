//{"name":"A. Счастливый?","group":"Codeforces - Codeforces Round #790 (Div. 4)","url":"https://codeforces.com/contest/1676/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n213132\n973894\n045207\n000000\n055776\n","output":"YES\nNO\nYES\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASchastlivii"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut n: i32 = input.read();

    let mut sum = 0;
    for i in 0..3 {
        sum += n % 10;
        n = n / 10;
    }
    for i in 0..3 {
        sum -= n % 10;
        n = n / 10;
    }

    if sum == 0 {
        out_line!("YES");
    } else {
        out_line!("NO");
    }
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
