use linspace::Linspace;
use std::thread;

fn is_euler_brick(a: u64, b: u64, c: u64) -> bool {
    // calc diagonals squared
    let d1: f64 = ((a * a + b * b) as f64).sqrt();
    let d2: f64 = ((a * a + c * c) as f64).sqrt();
    let d3: f64 = ((b * b + c * c) as f64).sqrt();

    (d1).fract() == 0.0 && (d2).fract() == 0.0 && (d3).fract() == 0.0
}

fn find_euler_brick(range: u64, threads: usize, break_after_one: bool) {
    // let a = 117u64;
    // let b = 44u64;
    // let c = 240u64;

    let mut break_loop: bool = false;
    let mut handles = vec![thread::spawn(|| {}); threads];

    for a in 1..range {
        for b in 1..range {
            // start multithreading in the third nest
            let section_idxs = (0u64..range).linspace(threads).collect();

            let mut handles: vec![];
            for i in 0..threads {
                handles[i] = thread::spawn(|| {
                    // thread body
                    let section_start = section_idxs[i];
                    let section_end = if i < threads - 1 {
                        section_idxs[i + 1]
                    } else {
                        range
                    };

                    for c in section_start as u64..section_end as u64 {
                        if is_euler_brick(a, b, c) {
                            if break_after_one {
                                break_loop = break_after_one;
                                break;
                            }

                            println!(
                                "Found non-perfect euler brick at a = {}, b = {}, c = {}",
                                a, b, c
                            );
                        }
                    }
                });
            }

            if break_loop {
                break;
            }
        }
        if break_loop {
            break;
        }
    }
}

fn main() {
    find_euler_brick(1000, 10, false);
}
