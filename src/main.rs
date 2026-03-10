use crate::util::{calc_euler_triangles, sort_triangles, store_triangles};
mod util;

fn main() {
    let range = (1, 10_u64.pow(6));
    let threads = 1;

    let mut triangles = calc_euler_triangles(range, threads);

    println!(
        "Found {} euler triangles in range {}, {}",
        triangles.len(),
        range.0,
        range.1
    );

    let triangles_sorted_a = sort_triangles(&triangles, false);
    let triangles_sorted_b = sort_triangles(&triangles, true);

    store_triangles(&triangles_sorted_a, "triangles_sorted_a.txt");
    store_triangles(&triangles_sorted_b, "triangles_sorted_b.txt");
}
