//{"name":"C. Корней Корнеевич и XOR (простая версия)","group":"Codeforces - CP3 DP Mashup","url":"https://codeforces.com/group/kgiEExP5XR/contest/376906/problem/C","interactive":false,"timeLimit":1500,"tests":[{"input":"4\n4 2 2 4\n","output":"4\n0 2 4 6\n"},{"input":"8\n1 0 1 7 12 5 3 2\n","output":"12\n0 1 2 3 4 5 6 7 10 11 12 13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CKorneiKorneevichIXORProstayaVersiya"}}}

use std::collections::HashSet;
use std::hash::Hash;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::seg_tree::{SegTree, SegTreeNode};
use algo_lib::xor_set::XorSet;
use algo_lib::{out, out_line};

#[derive(Clone, Copy)]
struct Node {
    max: usize,
}

impl SegTreeNode<usize> for Node {
    fn combine(lhr: &Self, rhs: &Self) -> Self {
        Self {
            max: std::cmp::max(lhr.max, rhs.max),
        }
    }

    fn zero() -> Self {
        Self { max: 0 }
    }

    fn leaf(data: &usize) -> Self {
        Self { max: *data }
    }
}

fn solve_brute_force(data: &Vec<usize>) {
    let mut dp: Vec<_> = (0..data.len())
        .map(|i| vec![0, i as u32].into_iter().collect::<HashSet<u32>>())
        .collect();

    for i in 0..data.len() {
        for j in 0..i {
            if data[j] < data[i] {
                dp[i] = dp[i].union(&dp[j]).cloned().collect::<HashSet<u32>>();
            }
        }
    }
}

fn solve_bottom_up(data: &Vec<usize>) -> Vec<u32> {
    let mut dp: Vec<XorSet> = vec![vec![0].into_iter().collect(); 501];
    let mut seg_tree = SegTree::<Node, usize>::new(vec![0; 501]);

    for x in data {
        let max_prefix = seg_tree.query(..x);
        seg_tree.update_data(*x, |n| *n = *x);
        if max_prefix.max > 0 {
            dp[*x] = dp[max_prefix.max]
                .range_add(*x as u32)
                .union(&dp[max_prefix.max]);
        } else {
            dp[*x].set_value(*x as u32);
        }
    }

    #[cfg(test)]
    {
        let mut sorted = data.clone();
        sorted.sort();
        sorted.dedup();
        for i in sorted {
            eprintln!("{} -> {:?}", i, (&dp[i]).into_iter().collect::<Vec<u32>>())
        }
    }

    dp.into_iter()
        .reduce(|x, y| x.union(&y))
        .map(|x| x.into_iter())
        .unwrap()
        .collect()
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let data = input.read_vec(n);
    let res = solve_bottom_up(&data);
    out_line!(res.len());
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

#[cfg(test)]
mod tests {
    use crate::solve_bottom_up;

    #[test]
    pub fn test1() {
        let data = vec![1, 500, 500];
        assert_eq!(solve_bottom_up(&data), vec![0, 1, 500, 501])
    }

    #[test]
    pub fn test2() {
        let data = vec![500];
        assert_eq!(solve_bottom_up(&data), vec![0, 500])
    }

    #[test]
    pub fn test3() {
        let data = vec![1, 2, 3, 500];
        assert_eq!(solve_bottom_up(&data), vec![0, 1, 2, 3, 500, 501, 502, 503])
    }

    #[test]
    pub fn test4() {
        let data = vec![1, 0, 1, 7, 12, 5, 3, 2];
        assert_eq!(
            solve_bottom_up(&data),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 10, 11, 12, 13]
        );
    }
    #[test]
    pub fn test5() {
        let data = vec![256, 1, 2, 255]; //, 5, 3, 2];
        assert_eq!(
            solve_bottom_up(&data),
            vec![0, 1, 2, 3, 252, 253, 254, 255, 256]
        );
    }

    #[test]
    pub fn test6() {
        let data = vec![256, 1, 2, 255]; //, 5, 3, 2];
        assert_eq!(
            solve_bottom_up(&data),
            vec![0, 1, 2, 3, 252, 253, 254, 255, 256]
        );
    }

    #[test]
    pub fn test7() {
        let mut data: Vec<_> = (0..=100000).map(|_| 0).collect();
        data[4] = 100;
        data[10] = 200;
        data[10000] = 500;
        assert_eq!(
            solve_bottom_up(&data),
            vec![0, 100, 172, 200, 316, 344, 400, 500]
        )
    }

    #[test]
    pub fn test8() {
        let data = vec![16, 64, 32, 128];
        assert_eq!(
            solve_bottom_up(&data),
            vec![0, 100, 172, 200, 316, 344, 400, 500]
        )
    }
}

//START MAIN
mod tester;

fn main() {
    // loop {
    //     let n = 10;
    //     let random_data: Vec<u32> = (0..10).map(|x| rand::random::<u32>() % 500).collect();
    //     let mut copy = random_data.clone();
    //     let set: XorSet = random_data.iter().cloned().collect();
    //     copy.sort();
    //     copy.dedup();
    //     assert_eq!(set.into_iter().collect::<Vec<u32>>(), copy);
    // }

    tester::run_tests();
}
//END MAIN
