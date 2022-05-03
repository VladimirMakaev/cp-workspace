//{"name":"B. Тройка","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n1\n1\n3\n2 2 2\n7\n2 2 3 3 4 2 2\n8\n1 4 3 4 3 2 4 1\n9\n1 1 1 2 2 2 3 3 3\n5\n1 5 2 4 3\n4\n4 4 4 4\n","output":"-1\n2\n2\n4\n3\n-1\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTroika"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let data: Vec<usize> = input.read_vec(n);
    let mut counter = vec![0; 200001];
    for x in data {
        counter[x] = counter[x] + 1;
        if counter[x] >= 3 {
            out_line!(x);
            return;
        }
    }
    out_line!(-1);
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
