use std::env;
use std::fs::File;
use std::io::{BufWriter, Error, ErrorKind, Write};
use tqdm::pbar;

const DEFAULT_RANGE: (u64, u64) = (1, 10_000);

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

fn find_euler_triangles(range: (u64, u64)) -> Vec<(u64, u64)> {
    assert!(range.0 <= range.1);

    let mut triangles: Vec<(u64, u64)> = Vec::new();

    let mut pbar = pbar(Some((range.1 - range.0) as usize));
    for a in range.0..range.1 {
        pbar.update(1).unwrap();
        for b in range.0..range.1 {
            if is_euler_triangle(a, b) {
                if !is_duplicate(a, b, &triangles) {
                    triangles.push((a, b));
                }
            }
        }
    }

    triangles
}

fn main() {
    let range = (1, 1_000_000);
    let triangles = find_euler_triangles(range);

    store_triangles(&triangles, "triangles.txt");
    println!(
        "Found {} euler triangles in range {}, {}",
        triangles.len(),
        range.0,
        range.1
    );
}
