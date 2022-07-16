fn main() {
    let grid_size = 20;
    let mut paths: u64 = 1;

    for i in 0..grid_size {
        paths *= (2 * grid_size) - i;
        paths /= i + 1;
    }

    println!("{}", paths)
}
