#[macro_use]
extern crate bencher;
extern crate rand;

use algo_lib::dsu::*;
use bencher::Bencher;

fn test1(bench: &mut Bencher) {
    let mut dsu_r = DSU::new(100000);
    bench.iter(|| {
        dsu_r.union(
            rand::random::<usize>() % 100000,
            rand::random::<usize>() % 100000,
        )
    });
}

benchmark_group!(benches, test1);
benchmark_main!(benches);
