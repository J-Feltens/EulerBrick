use tqdm::pbar;

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
    return false;
}

fn find_euler_triangles(a_range: (u64, u64), b_range: (u64, u64)) -> Vec<(u64, u64)> {
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

    return triangles;
}

fn main() {
    let range = (1, 10_000);

    let triangles = find_euler_triangles(range, range);

    println!("Found {} euler triangles:", triangles.len());
    for t in triangles.iter() {
        println!("  {}, {}, {}", t.0, t.1, (t.0 * t.0) + (t.1 * t.1).isqrt());
    }
}
