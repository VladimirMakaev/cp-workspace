//{"name":"F. Съедая конфеты","group":"Codeforces - Codeforces Round #784 (Div. 4)","url":"https://codeforces.com/contest/1669/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n10 20 10\n6\n2 1 4 2 4 1\n5\n1 2 4 8 16\n9\n7 3 20 5 15 1 11 8 10\n","output":"2\n6\n0\n7\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSedayaKonfeti"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve_rec(
    prefix: usize,
    prefix_sum: isize,
    suffix: usize,
    suffix_sum: isize,
    data: &Vec<isize>,
    dp: &mut Vec<Vec<isize>>,
) -> isize {
    if suffix + prefix >= data.len() {
        return (prefix_sum - suffix_sum).abs();
    }
    if dp[prefix][suffix] != isize::MAX {
        return dp[prefix][suffix];
    }

    let skip_right = solve_rec(
        prefix + 1,
        prefix_sum + data[prefix],
        suffix,
        suffix_sum,
        data,
        dp,
    );
    let skip_left = solve_rec(
        prefix,
        prefix_sum,
        suffix + 1,
        suffix_sum + data[data.len() - suffix - 1],
        data,
        dp,
    );
    let go_both = solve_rec(
        prefix + 1,
        prefix_sum + data[prefix],
        suffix + 1,
        suffix_sum + data[data.len() - suffix - 1],
        data,
        dp,
    );

    dp[prefix][suffix] = std::cmp::min(std::cmp::min(skip_left, skip_right), go_both);
    return dp[prefix][suffix];
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let data = input.read_vec(n);
    let mut dp = vec![vec![isize::MAX; n + 1]; n + 1];
    solve_rec(0, 0, 0, 0, &data, &mut dp);
    let mut result = 0;

    for alice in 0..=n {
        for bob in 0..=n {
            if dp[alice][bob] == 0 {
                result = std::cmp::max(result, alice + bob);
            }
        }
    }
    out_line!(result);
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
