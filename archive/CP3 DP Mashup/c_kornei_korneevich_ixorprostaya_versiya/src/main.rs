//{"name":"C. Корней Корнеевич и XOR (простая версия)","group":"Codeforces - CP3 DP Mashup","url":"https://codeforces.com/group/kgiEExP5XR/contest/376906/problem/C","interactive":false,"timeLimit":1500,"tests":[{"input":"4\n4 2 2 4\n","output":"4\n0 2 4 6\n"},{"input":"8\n1 0 1 7 12 5 3 2\n","output":"12\n0 1 2 3 4 5 6 7 10 11 12 13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CKorneiKorneevichIXORProstayaVersiya"}}}

use std::vec;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve_top_down(a: &Vec<u16>, dp: &mut Vec<Vec<Option<u16>>>, suffix: usize) {
    if suffix == a.len() {
        return;
    }

    let cur = a[suffix];

    if suffix == 0 {
        dp[0][0] = Some(0);
        dp[cur as usize][suffix] = Some(cur);
    } else {
        for xor in 0..1000 {
            if let Some(last_seen) = dp[xor][suffix - 1] {
                dp[xor][suffix] =
                    Some(dp[xor][suffix].map_or(last_seen, |prev| std::cmp::min(prev, last_seen)));

                if last_seen < cur {
                    dp[xor ^ cur as usize][suffix] = Some(
                        dp[xor ^ cur as usize][suffix].map_or(cur, |prev| std::cmp::min(cur, prev)),
                    );
                }
            }
        }
    }
    solve_top_down(a, dp, suffix + 1);
}

fn solve(input: &mut Input, _test_case: usize) {
    let k: usize = input.read();
    let a = input.read_vec::<u16>(k);
    let mut dp: Vec<Vec<Option<u16>>> = vec![vec![None; k]; 1000];
    solve_top_down(&a, &mut dp, 0);

    let mut result = Vec::new();
    for i in 0..1000 {
        if dp[i][k - 1].is_some() {
            result.push(i);
        }
    }
    out_line!(result.len());
    out_line!(result);
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
