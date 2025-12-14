fn combinations_with_repetition(n: u64, k: u64) -> u64 {
    let nn = n + k - 1;
    binomial(nn, k)
}

fn binomial(n: u64, k: u64) -> u64 {
    let k = k.min(n - k);
    let mut result = 1u64;

    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

fn is_euler_brick(a: u64, b: u64, c: u64) -> bool {
    // calc diagonals squared
    let d1: f64 = ((a * a + b * b) as f64).sqrt();
    let d2: f64 = ((a * a + c * c) as f64).sqrt();
    let d3: f64 = ((b * b + c * c) as f64).sqrt();

    (d1).fract() == 0.0 && (d2).fract() == 0.0 && (d3).fract() == 0.0
}

fn find_euler_brick(range: u64, break_after_one: bool) {
    // let a = 117u64;
    // let b = 44u64;
    // let c = 240u64;

    let mut bricks_checked: Vec<Vec<u64>> = vec![];

    let mut a: u64 = 1;
    let mut b: u64 = 1;
    let mut c: u64 = 1;
    let mut break_loop: bool = false;

    while a < range {
        while b < range {
            while c < range {
                bricks_checked.push(vec![a, b, c]);

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
                c += 1;
            }
            if break_loop {
                break;
            }
            c = 1;
            b += 1;
        }
        if break_loop {
            break;
        }
        b = 1;
        a += 1;
    }
}

fn main() {
    find_euler_brick(1000, false);
}
