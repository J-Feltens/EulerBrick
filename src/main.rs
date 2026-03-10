use crate::util::{calc_euler_triangles, sort_triangles, store_triangles};
use std::env;
mod util;

pub const DEBUG_MODE: bool = false;

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

    let triangles = calc_euler_triangles(range, threads);

    println!(
        "Found {} euler triangles in range {}, {}",
        triangles.len(),
        range.0,
        range.1
    );

    if DEBUG_MODE {
        let triangles_sorted_a = sort_triangles(&triangles, false);
        let triangles_sorted_b = sort_triangles(&triangles, true);
        store_triangles(&triangles_sorted_a, "triangles_sorted_a.txt");
        store_triangles(&triangles_sorted_b, "triangles_sorted_b.txt");
    }
}
