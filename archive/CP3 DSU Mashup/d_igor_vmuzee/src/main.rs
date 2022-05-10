//{"name":"D. Игорь в музее","group":"Codeforces - CP3 DSU Mashup","url":"https://codeforces.com/group/kgiEExP5XR/contest/380181/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5 6 3\n******\n*..*.*\n******\n*....*\n******\n2 2\n2 5\n4 3\n","output":"6\n4\n10\n"},{"input":"4 4 1\n****\n*..*\n*.**\n****\n3 2\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DIgorVMuzee"}}}

use algo_lib::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();
    let k: usize = input.read();
    let lines: Vec<String> = input.read_vec(n);
    let matrix: Vec<Vec<u8>> = lines
        .into_iter()
        .map(|line| line.as_bytes().iter().cloned().collect())
        .collect();

    let mut dsu = DSU::new(n * m);
    let mut result = vec![0; n * m];

    fn get_cell(i: usize, j: usize, m: usize) -> usize {
        return i * m + j;
    }

    for i in 1..n - 1 {
        for j in 1..m - 1 {
            result[get_cell(i, j, m)] = get_walls(&matrix, i, j);
        }
    }

    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 46 {
                let cur_cell = get_cell(i, j, m);
                let joins = get_join(&matrix, i, j);

                for (x, y) in joins {
                    let join_cell = get_cell(x, y, m);
                    if dsu.find(join_cell) != dsu.find(cur_cell) {
                        let wall_cur = result[dsu.find(cur_cell)];
                        let wall_join = result[dsu.find(join_cell)];

                        result[dsu.union(cur_cell, join_cell)] = wall_cur + wall_join;
                    }
                }
            }
        }
    }

    let queries: Vec<(usize, usize)> = input.read_vec(k);
    for (x, y) in queries {
        out_line!(result[dsu.find(get_cell(x - 1, y - 1, m))]);
    }
}

fn get_join(matrix: &Vec<Vec<u8>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    return vec![(i + 1, j), (i - 1, j), (i, j - 1), (i, j + 1)]
        .into_iter()
        .map(
            |(x, y)| match matrix.get(x).map(|row| row.get(y)).flatten() {
                Some(res) if res == &46 => Some((x, y)),
                _ => None,
            },
        )
        .flatten()
        .collect();
}

fn get_walls(matrix: &Vec<Vec<u8>>, i: usize, j: usize) -> usize {
    fn one_if_wall(matrix: &Vec<Vec<u8>>, i: usize, j: usize) -> usize {
        if let Some(x) = matrix.get(i).map(|x| x.get(j)).flatten() {
            if x == &42 {
                1
            } else {
                0
            }
        } else {
            0
        }
    }
    return one_if_wall(matrix, i + 1, j)
        + one_if_wall(matrix, i - 1, j)
        + one_if_wall(matrix, i, j + 1)
        + one_if_wall(matrix, i, j - 1);
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
