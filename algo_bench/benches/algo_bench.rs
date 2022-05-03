#[macro_use]
extern crate bencher;

use bencher::Bencher;

fn test1(bench: &mut Bencher) {}

benchmark_group!(benches, test1);
benchmark_main!(benches);
