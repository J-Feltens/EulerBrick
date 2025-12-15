use std::fs::File;
use std::io::{BufWriter, Write};
use std::thread;
use std::thread::JoinHandle;
use tqdm::pbar;

const DEFAULT_RANGE: (u64, u64) = (1, 10_000);

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

fn is_euler_triangle(a: u64, b: u64) -> bool {
    // calc diagonals squared
    let c: f64 = ((a * a + b * b) as f64).sqrt();

    c.fract() == 0.0
}

fn is_duplicate(a: u64, b: u64, triangles: &Vec<(u64, u64)>) -> bool {
    for triangle in triangles.iter() {
        if (triangle.0 == a && triangle.1 == b) || (triangle.0 == b && triangle.1 == a) {
            return true;
        }
    }
    false
}

fn store_triangles(triangles: &Vec<(u64, u64)>, path: &str) {
    let mut f = File::create(path).expect("Failed to write to file");
    let mut writer = BufWriter::new(f);

    writeln!(writer, "{}", triangles.len()).unwrap();

    for triangle in triangles.iter() {
        writeln!(writer, "{},{}", triangle.0, triangle.1).unwrap();
    }
}

fn find_euler_triangles_mt(a_range: (u64, u64), b_range: (u64, u64)) -> Vec<(u64, u64)> {
    assert!(a_range.0 <= a_range.1);
    assert!(b_range.0 <= b_range.1);

    let mut triangles: Vec<(u64, u64)> = Vec::new();

    let mut pbar = pbar(Some((a_range.1 - a_range.0) as usize));
    for a in a_range.0..a_range.1 {
        pbar.update(1).unwrap();
        for b in b_range.0..b_range.1 {
            if is_euler_triangle(a, b) {
                if !is_duplicate(a, b, &triangles) {
                    triangles.push((a, b));
                }
            }
        }
    }

    triangles
}

fn find_euler_triangles(range: (u64, u64), threads: usize) -> Vec<(u64, u64)> {
    let mut triangles = Vec::new();

    // setup multithreading
    let section_idxs = linspace(range.0, range.1, threads);
    let mut handles: Vec<JoinHandle<Vec<(u64, u64)>>> = Vec::new();

    for i in 0..section_idxs.len() {
        let range_from = section_idxs[i];
        let range_to = if i < section_idxs.len() - 1 {
            section_idxs[i + 1]
        } else {
            range.1
        };
        let handle = thread::spawn(move || {
            // spawn new compute thread
            find_euler_triangles_mt((range_from, range_to), range)
        });
        handles.push(handle);
    }

    for handle in handles {
        let mut triangles_partial = handle.join().unwrap();
        triangles.append(&mut triangles_partial);
    }

    triangles
}

fn main() {
    let range = (1, 1_000_000);
    let threads = 30;

    let triangles = find_euler_triangles(range, threads);

    store_triangles(&triangles, "triangles.txt");
    println!(
        "Found {} euler triangles in range {}, {}",
        triangles.len(),
        range.0,
        range.1
    );
}
