use ndarray::Array2;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::thread;
use tqdm::pbar;

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
        // .chunks_exact(2) safely slices the 1D array into pairs [a, b]
        for chunk in flat_data.chunks_exact(2) {
            // Because the chunk is exactly 2 items, LLVM knows chunk[0]
            // and chunk[1] are safe. Zero bounds-checking overhead!
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

pub fn run_multithreaded(max_side_length: usize, threads: usize) {
    let section_idxs = linspace(range.0, range.1, threads);
    let mut handles = Vec::new();

    for i in 0..threads {
        let range_from = section_idxs[i];
        let range_to = section_idxs[i + 1];
        let thread_id = i + 1;

        let is_first_thread = i == 0;

        let handle = thread::spawn(move || {
            calc_euler_triangles_stream(thread_id, (range_from, range_to), range, is_first_thread)
        });
        handles.push(handle);
    }

    // Wait for all threads to finish writing their files
    for handle in handles {
        handle.join().unwrap();
    }
}

// pub fn concat_files(threads: usize, output_file_path: &str) {
//     let f = File::create(output_file_path).expect("Failed to create output file");
//     let mut writer = BufWriter::new(f);
//
//     println!("Merging {} files into {}...", threads, output_file_path);
//     let mut progress_bar = pbar(Some(threads));
//
//     for i in 1..=threads {
//         let file_name = format!("results/tmp/triangles_part_{}.txt", i);
//         if let Ok(part_file) = File::open(&file_name) {
//             let mut reader = BufReader::new(part_file);
//             std::io::copy(&mut reader, &mut writer).expect("Failed to copy data");
//         } else {
//             eprintln!("Warning: Could not find {}", file_name);
//         }
//
//         progress_bar.update(1).ok();
//     }
// }
