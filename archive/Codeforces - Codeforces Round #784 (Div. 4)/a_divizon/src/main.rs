//{"name":"A. Дивизон?","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n-789\n1299\n1300\n1399\n1400\n1679\n2300\n","output":"Division 4\nDivision 4\nDivision 4\nDivision 4\nDivision 3\nDivision 2\nDivision 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADivizon"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let rating: isize = input.read();
    out_line!(match rating {
        _ if rating >= 1900 => "Division 1",
        _ if rating >= 1600 => "Division 2",
        _ if rating >= 1400 => "Division 3",
        _ if rating <= 1399 => "Division 4",
        _ => todo!(),
    });
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
