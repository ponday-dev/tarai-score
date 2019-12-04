use std::time::Instant;

fn tarai(x: i32, y: i32, z: i32) -> i32 {
    if x <= y {
        return y;
    }
    return tarai(
        tarai(x - 1, y, z),
        tarai(y - 1, z, x),
        tarai(z - 1, x, y)
    );
}

fn main() {
    let start = Instant::now();
    println!("{}", tarai(15, 5, 0));
    let end = start.elapsed();
    println!("{}.{:03}s", end.as_secs(), end.subsec_nanos() / 1_000_000);
}

