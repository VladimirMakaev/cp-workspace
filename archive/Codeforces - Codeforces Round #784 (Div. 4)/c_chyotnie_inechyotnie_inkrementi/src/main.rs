//{"name":"C. Чётные и нечётные инкременты","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 2 1\n4\n2 2 2 3\n4\n2 2 2 2\n5\n1000 1 1000 1 1000\n","output":"YES\nNO\nYES\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CChyotnieINechyotnieInkrementi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let data: Vec<usize> = input.read_vec(n);
    let mut odd_even_c = 0;
    let mut odd_odd_c = 0;
    let mut even_even_c = 0;
    let mut even_odd_c = 0;

    for i in 0..n {
        match (i % 2 + 1, data[i]) {
            (index_odd, value_odd) if index_odd % 2 == 1 && value_odd % 2 == 1 => {
                odd_odd_c += 1;
            }
            (index_odd, value_even) if index_odd % 2 == 1 && value_even % 2 == 0 => {
                odd_even_c += 1;
            }
            (index_even, value_odd) if index_even % 2 == 0 && value_odd % 2 == 1 => {
                even_odd_c += 1;
            }
            (index_even, value_even) if index_even % 2 == 0 && value_even % 2 == 0 => {
                even_even_c += 1;
            }
            (_, _) => panic!(),
        }
    }

    let yes_even = even_even_c == 0 || even_odd_c == 0;
    let yes_odd = odd_odd_c == 0 || odd_even_c == 0;
    out_line!(if yes_even && yes_odd { "YES" } else { "NO" });
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
