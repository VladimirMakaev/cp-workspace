//{"name":"F. Майк и футы","group":"Codeforces - CP3 DSU Mashup","url":"https://codeforces.com/group/kgiEExP5XR/contest/380181/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1 2 3 4 5 4 3 2 1 6\n","output":"6 4 4 3 3 2 2 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMaikIFuti"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
mod seg_tree {
    #[derive(Clone)]
    pub struct Node {
        max: usize,
        update_pending: bool,
    }

    impl Node {
        pub fn zero() -> Self {
            Self {
                max: 0,
                update_pending: false,
            }
        }

        pub fn leaf(data: usize) -> Self {
            Self {
                max: data,
                update_pending: false,
            }
        }
    }

    pub struct SegTree {
        tree: Vec<Node>,
        data: Vec<usize>,
        size: usize,
    }

    impl SegTree {
        pub fn new(data: Vec<usize>) -> Self {
            Self {
                tree: vec![Node::zero(); 4 * data.len()],
                size: data.len(),
                data: data,
            }
        }

        fn left(cur: usize) -> usize {
            return cur << 1;
        }

        fn right(cur: usize) -> usize {
            return cur << 1 + 1;
        }

        fn build_internal(&mut self, cur: usize, l: usize, r: usize) {
            if l == r {
                self.tree[cur] = Node::leaf(self.data[l]);
            } else {
                let mid = (l + r) / 2;
                self.build_internal(Self::left(cur), l, mid);
                self.build_internal(Self::right(cur), mid + 1, r);
            }
        }

        pub fn build(&mut self) {
            self.build_internal(1, 0, self.size - 1);
        }

        fn next_iteration(&mut self, cur: usize, l: usize, r: usize, lx: usize, rx: usize) {
            if rx < l || lx > r {
                return;
            }

            if lx >= l && rx <= r {}
        }

        pub fn next(&mut self, l: usize, r: usize) {
            self.next_iteration(1, l, r, 0, self.size - 1)
        }

        fn query_range_internal(
            &mut self,
            cur: usize,
            l: usize,
            r: usize,
            lx: usize,
            rx: usize,
        ) -> &Node {
            if r < lx || l > rx {
                return &Node::zero();
            }
            if l >= lx && r <= rx {
                return &self.tree[cur];
            } else {
                let mid = (lx + rx) / 2;
                let left = self.query_range_internal(Self::left(cur), l, r, lx, mid);
                let right = self.query_range_internal(Self::right(cur), l, r, mid + 1, rx);

                todo!()
            }
        }

        pub fn query_range(&mut self, l: usize, r: usize) -> usize {
            todo!()
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test1() {}
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let data: Vec<usize> = input.read_vec(n);
    let mut tree = seg_tree::SegTree::new(data);
    tree.build();
    let mut result = Vec::with_capacity(n);
    result.push(tree.query_range(0, n - 1));

    for i in 1..n {
        tree.next(i, n - 1);
        result.push(tree.query_range(i, n - 1));
    }

    out!(result);
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
