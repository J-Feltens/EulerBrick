use crate::util::{get_problem_part, solve_problem_part};
use std::cmp::max;
use std::env;
mod util;
use tqdm::tqdm;

pub const OUTPUT_FILE_NAME: &str = "triangles.txt";
pub const OUTPUT_FILE_PATH: &str = "results/";

pub const DEFAULT_THREADS: usize = 1;
pub const DEFAULT_RANGE: u64 = 10_u64.pow(3);

fn parse_args() -> (usize, u64) {
    // Collect command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Set your default values
    let mut threads = DEFAULT_THREADS;
    let mut max_side_length = DEFAULT_RANGE;

    // Parse thread count if provided
    if args.len() >= 3 {
        threads = args[1]
            .parse::<usize>()
            .expect("Error: Thread count must be a positive integer");
    }

    // Parse max side length if provided
    if args.len() >= 3 {
        max_side_length = args[2]
            .parse::<u64>()
            .expect("Error: Max side length must be a positive integer");
    } else if args.len() == 3 {
        println!(
            "Warning: You provided a start range but no end range. Using default range instead."
        );
    }

    (threads, max_side_length)
}
fn main() {
    let (threads, max_side_length) = parse_args();

    for part in tqdm(0..max_side_length) {
        let problem = get_problem_part(max_side_length as usize, part as usize);
        let problem_size = problem.nrows();

        solve_problem_part(problem);
    }
}
