//{"name":"B. Оптимизация дорог","group":"Codeforces - CP3 DP Mashup","url":"https://codeforces.com/group/kgiEExP5XR/contest/376906/problem/B","interactive":false,"timeLimit":3000,"tests":[{"input":"4 10 0\n0 3 4 8\n5 8 3 6\n","output":"47\n"},{"input":"4 10 2\n0 3 4 8\n5 8 3 6\n","output":"38\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOptimizatsiyaDorog"}}}

use std::collections::{HashMap, HashSet};
use std::{usize, vec};

use algo_lib::compress::SparseIndex;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

pub fn solve_memory_trick(n: usize, k: usize, speeds: &Vec<u32>, signs: &Vec<u32>) -> u32 {
    let mut dp = vec![vec![vec![u32::MAX; n + 2]; n + 2]; 2];
    dp[0][n - 1][n] = (signs[n] - signs[n - 1]) * speeds[n - 1];
    for suffix in (1..n).rev() {
        dp[0][suffix - 1][suffix] =
            speeds[suffix - 1] * (signs[suffix] - signs[suffix - 1]) + dp[0][suffix][suffix + 1];
    }

    for i in 1..=n {
        eprint!("{} ", dp[0][i - 1][i]);
    }

    for i in 1..=k {
        eprintln!();
        _solve(1, 0, i, n, speeds, signs, &mut dp);
        dp[(i & 1) ^ 1] = vec![vec![u32::MAX; n + 1]; n + 1];
        for i in 1..=n {
            eprint!("{} ", dp[i & 1][i - 1][i]);
        }
    }

    fn _solve(
        suffix: usize,
        last_seen: usize,
        k: usize,
        n: usize,
        speeds: &Vec<u32>,
        signs: &Vec<u32>,
        dp: &mut Vec<Vec<Vec<u32>>>,
    ) -> u32 {
        if suffix == n + 1 {
            return 0;
        }

        if dp[k & 1][last_seen][suffix] < u32::MAX / 2 {
            return dp[k & 1][last_seen][suffix];
        }

        let distance = signs[suffix] - signs[suffix - 1];

        if k > 0 {
            dp[k & 1][last_seen][suffix] = speeds[last_seen] * distance
                + _solve(suffix + 1, last_seen, k - 1, n, speeds, signs, dp)
        }
        dp[k & 1][last_seen][suffix] = speeds[last_seen] * distance
            + std::cmp::min(
                dp[k & 1][last_seen][suffix],
                _solve(suffix + 1, suffix, k, n, speeds, signs, dp),
            );

        return dp[k & 1][last_seen][suffix];
    }

    return _solve(1, 0, k, n, speeds, signs, &mut dp);
}

pub fn solve_top_down_rec(
    suffix: u32,
    start_speed: u32,
    k: u32,
    n: u32,
    speeds: &Vec<u32>,
    signs: &Vec<u32>,
    dp: &mut Vec<Vec<Vec<Option<u32>>>>,
    index: &SparseIndex<u32>,
) -> u32 {
    let distance = signs[suffix as usize] - signs[(suffix - 1) as usize];

    let speed_index = index.compress_unw(&start_speed);

    if suffix == n {
        dp[suffix as usize][speed_index as usize][k as usize] = Some(distance as u32 * start_speed);
        return distance as u32 * start_speed;
    }

    if let Some(res) = dp[suffix as usize][speed_index as usize][k as usize] {
        return res;
    }

    let keep_sign = solve_top_down_rec(
        suffix + 1,
        speeds[suffix as usize],
        k,
        n,
        speeds,
        signs,
        dp,
        index,
    );

    if k == 0 {
        dp[suffix as usize][speed_index as usize][k as usize] =
            Some(distance * start_speed + keep_sign);
    } else {
        let remove_sign =
            solve_top_down_rec(suffix + 1, start_speed, k - 1, n, speeds, signs, dp, index);

        dp[suffix as usize][speed_index as usize][k as usize] =
            Some(distance * start_speed + std::cmp::min(keep_sign, remove_sign));
    }

    return dp[suffix as usize][speed_index as usize][k as usize].unwrap();
}

pub fn solve_top_down_rec_2(
    suffix: u32,
    start_speed: u32,
    k: u32,
    n: u32,
    speeds: &Vec<u32>,
    signs: &Vec<u32>,
    dp: &mut Vec<HashMap<(usize, u32), u32>>,
    index: &SparseIndex<u32>,
) -> u32 {
    let distance = signs[suffix as usize] - signs[(suffix - 1) as usize];

    let speed_index = index.compress_unw(&start_speed);

    if suffix == n {
        dp[suffix as usize].insert((speed_index, k), distance as u32 * start_speed);
        return distance as u32 * start_speed;
    }

    if let Some(res) = dp[suffix as usize].get(&(speed_index, k)) {
        return *res;
    }

    let keep_sign = solve_top_down_rec_2(
        suffix + 1,
        speeds[suffix as usize],
        k,
        n,
        speeds,
        signs,
        dp,
        index,
    );

    if k == 0 {
        dp[suffix as usize].insert((speed_index, k), distance * start_speed + keep_sign);
    } else {
        let remove_sign =
            solve_top_down_rec_2(suffix + 1, start_speed, k - 1, n, speeds, signs, dp, index);

        dp[suffix as usize].insert(
            (speed_index, k),
            distance * start_speed + std::cmp::min(keep_sign, remove_sign),
        );
    }

    return dp[suffix as usize][&(speed_index, k)];
}

pub fn solve_top_down(n: u32, k: u32, speeds: &Vec<u32>, signs: &Vec<u32>) -> u32 {
    let start_speed = speeds[0];
    let mut dp = vec![HashMap::with_capacity(k as usize + 1); n as usize + 1];
    let index = SparseIndex::new(speeds.clone());
    let res = solve_top_down_rec_2(1, start_speed, k, n, speeds, signs, &mut dp, &index);
    //print_dp(&dp, n, k);
    res
}

fn solve_brute_force(n: usize, k: usize, speeds: &Vec<u32>, signs: &Vec<u32>) -> u32 {
    let mut min = u32::MAX;
    let mut vec: Vec<usize> = vec![];
    for mask in 0..(1 << (n + 1)) {
        let mut to_remove = HashSet::new();
        for i in 1..n {
            if mask & (1usize << i) > 0 {
                to_remove.insert(i);
            }
        }
        if to_remove.len() <= k {
            let mut result = 0;
            let mut speed = speeds[0];
            for i in 1..=n {
                result = result + speed * (signs[i] - signs[i - 1]);
                if !to_remove.contains(&i) && i < n {
                    speed = speeds[i];
                }
            }
            min = std::cmp::min(min, result);
            vec = to_remove.iter().cloned().collect();
        }
    }
    eprintln!("{:?}", vec);
    min
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, distance, k) = input.read();
    let mut signs: Vec<u32> = input.read_vec(n);
    signs.push(distance);
    let speed: Vec<u32> = input.read_vec(n);
    out!(solve_memory_trick(n, k, &speed, &signs));
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

#[cfg(test)]
mod tests {
    use super::solve_brute_force;
    use super::solve_memory_trick;
    use super::solve_top_down;

    #[test]
    pub fn memory_1() {
        assert_eq!(
            solve_memory_trick(4, 0, &vec![5, 8, 3, 6], &vec![0, 3, 4, 8, 10]),
            47
        );
    }
    #[test]
    pub fn memory_2() {
        assert_eq!(
            solve_memory_trick(4, 1, &vec![5, 8, 3, 6], &vec![0, 3, 4, 8, 10]),
            38
        );
    }

    #[test]
    pub fn new_test1() {
        assert_eq!(
            solve_top_down(4, 0, &vec![5, 8, 3, 6], &vec![0, 3, 4, 8, 10]),
            47
        );
    }

    #[test]
    pub fn new_test2() {
        assert_eq!(
            solve_top_down(4, 1, &vec![5, 8, 3, 6], &vec![0, 3, 4, 8, 10]),
            41
        );
    }

    #[test]
    pub fn new_test_3() {
        assert_eq!(solve_top_down(1, 0, &vec![5], &vec![0, 5]), 25);
    }

    #[test]
    pub fn new_test_4() {
        assert_eq!(
            solve_top_down(4, 1, &vec![2, 9, 7, 10], &vec![0, 2, 4, 6, 8, 10]),
            42
        )
    }

    #[test]
    pub fn test1() {
        assert_eq!(
            solve_top_down(4, 2, &vec![5, 8, 3, 6], &vec![0, 3, 4, 8, 10]),
            38
        );
        assert_eq!(
            solve_brute_force(4, 2, &vec![5, 8, 3, 6], &vec![0, 3, 4, 8, 10]),
            38
        );
    }
    #[test]

    pub fn test2() {
        assert_eq!(
            solve_top_down(4, 4, &vec![1, 8, 3, 6], &vec![0, 3, 4, 8, 10]),
            10
        );
        assert_eq!(
            solve_top_down(4, 4, &vec![1, 8, 3, 6], &vec![0, 3, 4, 8, 10]),
            10
        );
        assert_eq!(
            solve_brute_force(4, 4, &vec![1, 8, 3, 6], &vec![0, 3, 4, 8, 10]),
            10
        );
    }

    #[test]
    pub fn test3() {
        assert_eq!(
            solve_top_down(4, 2, &vec![1, 8, 3, 6], &vec![0, 3, 4, 8, 10]),
            20
        );
    }

    #[test]
    pub fn test4() {
        assert_eq!(
            solve_top_down(4, 1, &vec![1, 8, 3, 6], &vec![0, 5, 10, 15, 20]),
            55
        );
        assert_eq!(
            solve_top_down(4, 1, &vec![1, 8, 3, 6], &vec![0, 5, 10, 15, 20]),
            55
        );
    }

    #[test]
    pub fn test5() {
        assert_eq!(
            solve_top_down(4, 2, &vec![1, 2, 3, 4], &vec![0, 1, 2, 3, 4]),
            7
        );
        assert_eq!(
            solve_top_down(4, 2, &vec![1, 2, 3, 4], &vec![0, 1, 2, 3, 4]),
            7
        );
    }

    #[test]
    pub fn test6() {
        assert_eq!(
            solve_brute_force(4, 2, &vec![5, 10, 10, 17], &vec![0, 2, 4, 6, 8]),
            60
        );
        assert_eq!(
            solve_top_down(4, 2, &vec![5, 10, 10, 17], &vec![0, 2, 4, 6, 8]),
            60
        );
    }
    #[test]
    pub fn test7() {
        let t_down = solve_top_down(4, 2, &vec![10, 12, 14, 20], &vec![0, 2, 4, 6, 8]);
        let brute = solve_brute_force(4, 2, &vec![10, 12, 14, 20], &vec![0, 2, 4, 6, 8]);
        assert_eq!(t_down, brute);
    }

    #[test]
    pub fn test8() {
        let t_down = solve_top_down(4, 2, &vec![6, 9, 11, 16], &vec![0, 2, 4, 6, 8]);
        let brute = solve_brute_force(4, 2, &vec![6, 9, 11, 16], &vec![0, 2, 4, 6, 8]);
        assert_eq!(t_down, brute);
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    return;
    while true {
        let size = 5;
        let remove = rand::random::<usize>() % size;
        let speeds: Vec<u32> = (0..size)
            .map(|_| rand::random::<u32>() % 10 + 1)
            .into_iter()
            .collect();
        let signs = vec![0, 2, 4, 6, 8, 10, 15, 20, 30, 40, 50, 60, 70, 80, 90, 100];

        assert_eq!(
            solve_top_down(size as u32, remove as u32, &speeds, &signs),
            solve_memory_trick(
                size as usize,
                remove as usize,
                &speeds.clone(),
                &signs.clone()
            ) as u32,
            "{:?}, {:?}",
            speeds,
            signs
        )
    }
}
//END MAIN
