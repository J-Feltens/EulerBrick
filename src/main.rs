use std::thread;

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

fn find_euler_brick(range: u64, threads: usize, break_after_one: bool) {
    // let a = 117u64;
    // let b = 44u64;
    // let c = 240u64;

    let mut break_loop: bool = false;

    for a in 1..range {
        for b in 1..range {
            for c in 1..range {
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
