use std::fs::File;
use std::io::{BufWriter, Write};
use std::thread;
use tqdm::pbar;

fn linspace(start: u64, end: u64, steps: usize) -> Vec<u64> {
    assert!(steps > 0);
    assert!(start < end);
    let range = end - start;
    let mut v = Vec::with_capacity(steps);
    for i in 0..steps {
        v.push(start + (range * i as u64) / steps as u64);
    }
    v.push(end); // Ensure the exact end bound is included in our sections
    v
}

fn is_euler_triangle(a: u64, b: u64) -> bool {
    let c: f64 = ((a * a + b * b) as f64).sqrt();
    c.fract() == 0.0
}

fn calc_euler_triangles_stream(
    thread_id: usize,
    a_range: (u64, u64),
    global_max: u64,
    print_pbar: bool,
) {
    // 1. Give each thread its own dedicated file
    let file_name = format!("triangles_part_{}.txt", thread_id);
    let f = File::create(&file_name).expect("Failed to create file");

    // 2. Wrap it in a BufWriter so it writes in large, fast memory chunks
    let mut writer = BufWriter::new(f);

    let mut progress_bar = if print_pbar {
        Some(pbar(Some((a_range.1 - a_range.0) as usize)))
    } else {
        None
    };

    for a in a_range.0..a_range.1 {
        if let Some(ref mut pb) = progress_bar {
            pb.update(1).ok();
        }

        // 3. THE MAGIC TRICK: Start `b` at `a` instead of `b_range.0`.
        // This inherently prevents duplicates like (4, 3) if we already found (3, 4).
        for b in a..global_max {
            if is_euler_triangle(a, b) {
                // Stream directly to the file buffer, no massive Vec required
                writeln!(writer, "{},{}", a, b).unwrap();
            }
        }
    }

    // BufWriter flushes to disk automatically when it drops at the end of the function
}

pub fn run_multithreaded(range: (u64, u64), threads: usize) {
    let section_idxs = linspace(range.0, range.1, threads);
    let mut handles = Vec::new();

    for i in 0..threads {
        let range_from = section_idxs[i];
        let range_to = section_idxs[i + 1];
        let global_max = range.1;
        let thread_id = i + 1;

        // Only print the progress bar for the first thread to avoid console garble
        let is_first_thread = i == 0;

        let handle = thread::spawn(move || {
            calc_euler_triangles_stream(
                thread_id,
                (range_from, range_to),
                global_max,
                is_first_thread,
            )
        });
        handles.push(handle);
    }

    // Wait for all threads to finish writing their files
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    let range = (1, 10_000);
    let threads = 4;

    println!(
        "Calculating and writing directly to disk using {} thread(s)...",
        threads
    );

    run_multithreaded(range, threads);

    println!("Done! Check the generated triangles_part_X.txt files.");
}
