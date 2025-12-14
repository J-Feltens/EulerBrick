use std::os::unix::thread::JoinHandleExt;
use std::thread;
use std::thread::JoinHandle;

fn linspace(start: u64, end: u64, steps: usize) -> Vec<u64> {
    /*
       generates array [start, end)
    */
    assert!(steps > 0);
    assert!(start < end);

    let range = end - start;
    let mut v = Vec::with_capacity(steps);

    for i in 0..steps {
        // integer interpolation
        let value = start + (range * i as u64) / steps as u64;
        v.push(value);
    }

    v
}

fn is_euler_brick(a: u64, b: u64, c: u64) -> bool {
    // calc diagonals squared
    let d1: f64 = ((a * a + b * b) as f64).sqrt();
    let d2: f64 = ((a * a + c * c) as f64).sqrt();
    let d3: f64 = ((b * b + c * c) as f64).sqrt();

    (d1).fract() == 0.0 && (d2).fract() == 0.0 && (d3).fract() == 0.0
}

fn find_euler_brick_mt_special(a_range: (u64, u64), bc_range: u64) {
    for a in a_range.0..a_range.1 {
        for b in 1..bc_range {
            for c in 1..bc_range {
                if is_euler_brick(a, b, c) {
                    println!(
                        "Found non-perfect euler brick at a = {}, b = {}, c = {}",
                        a, b, c
                    );
                }
            }
        }
    }
}
fn find_euler_brick(range: u64, threads: usize) {
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(threads);
    let section_idxs = linspace(1, range, threads);

    for section in 0..threads {
        let start_idx = section_idxs[section];
        let end_idx = if section < threads - 1 {
            section_idxs[section + 1]
        } else {
            range
        };

        let handle = thread::spawn(move || {
            // spawn new compute thread
            find_euler_brick_mt_special((start_idx, end_idx), range);
        });
        handles.push(handle);

        println!(
            "started thread {}/{} to compute interval [{}, {}]",
            section + 1,
            threads,
            start_idx,
            end_idx
        );
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    find_euler_brick(1000, 3);
}
