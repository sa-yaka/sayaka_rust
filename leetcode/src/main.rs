mod solution;

use solution::*;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    //---------

    let num = vec![2, 7, 11, 15];
    let tow_num = two_sum(num, 9);
    println!("{:?}", tow_num);

    //---------
    let elapsed_time = now.elapsed();
    println!(
        "Running slow_function() took {} micros.",
        elapsed_time.as_micros()
    );
}
