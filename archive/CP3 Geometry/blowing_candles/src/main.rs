//{"name":"blowing_candles","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"blowing_candles"}}}

use algo_lib::geo::{convex_hull, diameter, Point};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let r: usize = input.read();
    let points = input
        .read_vec::<(isize, isize)>(n)
        .into_iter()
        .map(|(x, y)| Point::new(x, y))
        .collect::<Vec<_>>();

    out!((diameter(&convex_hull(&points))));
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
