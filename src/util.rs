use ndarray::Array2;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::thread;
use tqdm::tqdm;

fn linspace(start: u64, end: u64, steps: usize) -> Vec<u64> {
    assert!(steps > 0);
    assert!(start < end);
    let range = end - start;
    let mut v = Vec::with_capacity(steps);
    for i in 0..steps {
        v.push(start + (range * i as u64) / steps as u64);
    }
    v.push(end);
    v
}

pub fn get_problem_part(max_side_length: usize, part: usize) -> Array2<u64> {
    // returns a "problem part", i.e. one column from the rectangle problem space
    // e.g. for max_side_length = 6, part = 1:
    // [[5, 5],
    //  [6, 5],
    //  [2, 2],
    //  [3, 2],
    //  [4, 2],
    //  [5, 2],
    //  [6, 2]]
    //
    // additionally squares each entry, so actually returns
    // [[25, 25],
    //  [36, 25],
    //  [ 4,  4],
    //  [ 9,  4],
    //  [16,  4],
    //  [25,  4],
    //  [36,  4]]
    let size = max_side_length + 1;

    Array2::from_shape_fn((size, 2), |(row, col)| {
        if row <= part {
            if col == 0 {
                (max_side_length - part + row) as u64 * (max_side_length - part + row) as u64
            } else {
                (max_side_length - part) as u64 * (max_side_length - part) as u64
            }
        } else {
            if col == 0 {
                row as u64 * row as u64
            } else {
                (part + 1) as u64 * (part + 1) as u64
            }
        }
    })
}

pub fn is_euler_triangle(a_sq: u64, b_sq: u64) -> bool {
    // checks if the right-angled triangle described by
    // sqrt(a) and sqrt(b) is an a euler triangle
    let c: f64 = (a_sq as f64 + b_sq as f64).sqrt();
    c.fract() == 0.0
}

pub fn solve_problem_part(problem_part: Array2<u64>) {
    if let Some(flat_data) = problem_part.as_slice() {
        for chunk in flat_data.chunks_exact(2) {
            let a_sq = chunk[0];
            let b_sq = chunk[1];

            if is_euler_triangle(a_sq, b_sq) {
                // println!("Match found: a={}, b={}", a_sq.isqrt(), b_sq.isqrt());
            }
        }
    } else {
        println!("Memory was not contiguous!");
    }
}
pub fn distribute_and_solve(max_side_length: u64, thread_count: usize) {
    let parts = max_side_length as usize;

    let indices: Vec<usize> = (0..parts).collect();

    // 2. Calculate the chunk size (rounding up to ensure we cover everything)
    let chunk_size = (parts + thread_count - 1) / thread_count;

    // 3. Create a thread scope
    thread::scope(|s| {
        // .chunks() automatically slices the Vec into safe, non-overlapping arrays
        for chunk in indices.chunks(chunk_size) {
            // Spawn a thread for this specific chunk
            s.spawn(move || {
                // Loop through the sub-sequence assigned to this thread
                for &part_idx in chunk {
                    let problem_part = get_problem_part(max_side_length as usize, part_idx);
                    solve_problem_part(problem_part);
                }
            });
        }
    });
}
