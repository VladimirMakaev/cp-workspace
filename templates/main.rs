//$JSON

use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

$SOLVE

#[allow(unused)]
pub fn submit() -> bool {
    let io = $IO_SETTINGS;

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
