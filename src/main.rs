#[allow(dead_code)]

mod puzzles;
mod today;

fn main() {
    let now = std::time::Instant::now();
    println!(
        "Puzzle 1: {} ({}ms)",
        today::run_one(today::INPUT),
        now.elapsed().as_millis()
    );
    let now = std::time::Instant::now();
    println!(
        "Puzzle 2: {} ({}ms)",
        today::run_two(today::INPUT),
        now.elapsed().as_millis()
    );
}
