#![allow(warnings)]

use std::env;
mod util;
use crate::util::run_ultimate_speed;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut max_side_length = 1000 as u64;

    if args.len() >= 2 {
        max_side_length = args[1]
            .parse::<u64>()
            .expect("Error: Max side length count must be a positive integer");
    } else {
        println!("No max side length specified, defaulting to 1000");
    }

    run_ultimate_speed(max_side_length, max_side_length);
}
