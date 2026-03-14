use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

pub fn is_euler_triangle(a: u64, b: u64) -> bool {
    let c: f64 = ((a * a + b * b) as f64).sqrt();
    c.fract() == 0.0
}

pub fn run_ultimate_speed(max_a: u64, max_b: u64) {
    let total_matches = AtomicU64::new(0);

    // Rayon automatically chunks the 'a' loop perfectly across all CPU cores.
    // Zero manual thread scoping, zero linspace calculations.
    (1..=max_a).into_par_iter().for_each(|a| {
        let mut local_matches = 0;

        // The inner loop just hums along in CPU registers
        for b in 1..=max_b {
            if is_euler_triangle(a, b) {
                local_matches += 1;
            }
        }

        if local_matches > 0 {
            total_matches.fetch_add(local_matches, Ordering::Relaxed);
        }
    });

    println!(
        "Total Euler triangles found: {}",
        total_matches.load(Ordering::Relaxed)
    );
}
