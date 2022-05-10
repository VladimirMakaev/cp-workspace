//{"name":"C. Самые похожие слова","group":"Codeforces - Codeforces Round #790 (Div. 4)","url":"https://codeforces.com/contest/1676/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n2 4\nbest\ncost\n6 3\nabb\nzba\nbef\ncdu\nooo\nzzz\n2 7\naaabbbc\nbbaezfe\n3 2\nab\nab\nab\n2 8\naaaaaaaa\nzzzzzzzz\n3 1\na\nu\ny\n","output":"11\n8\n35\n0\n200\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSamiePokhozhieSlova"}}}

use std::usize;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn diff_bytes(left: u8, right: u8) -> usize {
    if left > right {
        (left - right) as usize
    } else {
        (right - left) as usize
    }
}

fn distance(left: &String, right: &String) -> usize {
    left.bytes()
        .zip(right.bytes())
        .map(|(x, y)| diff_bytes(x, y))
        .sum::<usize>()
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m) = input.read::<(usize, usize)>();
    let words: Vec<String> = input.read_vec(n);
    let mut min = usize::MAX;

    for i in 0..n {
        for j in i + 1..n {
            min = std::cmp::min(min, distance(&words[i], &words[j]));
        }
    }
    out_line!(min);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let vec = vec!["aaa".to_owned(), "zba".to_owned()];
        assert_eq!(distance(&vec[0], &vec[1]), 26);
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
