fn main() {
    let start = std::time::Instant::now();
    let _ = jiff::Zoned::now();
    eprintln!("elapsed={:?}", start.elapsed());
}
