use crate::util::run_multithreaded;
use std::env;
mod util;

pub const VERBOSE: bool = false;

pub const DEFAULT_THREADS: usize = 1;
pub const DEFAULT_RANGE: (u64, u64) = (1, 10_u64.pow(3));

fn parse_args() -> (usize, (u64, u64)) {
    // Collect command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Set your default values
    let mut threads = DEFAULT_THREADS;
    let mut range = DEFAULT_RANGE;

    // Parse thread count if provided
    if args.len() >= 3 {
        threads = args[1]
            .parse::<usize>()
            .expect("Error: Thread count must be a positive integer");
    }

    // Parse range (start and end) if provided
    if args.len() >= 4 {
        let start = args[2]
            .parse::<u64>()
            .expect("Error: Range start must be a positive integer");
        let end = args[3]
            .parse::<u64>()
            .expect("Error: Range end must be a positive integer");
        range = (start, end);
    } else if args.len() == 3 {
        println!(
            "Warning: You provided a start range but no end range. Using default range instead."
        );
    }

    (threads, range)
}
fn main() {
    let (threads, range) = parse_args();

    run_multithreaded(range, threads);

    println!(
        "Calculating and writing directly to disk using {} thread(s)...",
        threads
    );

    run_multithreaded(range, threads);

    println!("Done! Check the generated triangles_part_X.txt files.");
}
