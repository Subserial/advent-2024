#[allow(dead_code)]
mod puzzles;
mod today;

fn main() {
    let input = if cfg!(feature = "test-input") {
        println!("=== TEST INPUT ===");
        today::TEST
    } else {
        today::INPUT
    };
    let now = std::time::Instant::now();
    println!(
        "Puzzle 1: {} ({}ms)",
        today::run_one(input),
        now.elapsed().as_millis()
    );
    let now = std::time::Instant::now();
    println!(
        "Puzzle 2: {} ({}ms)",
        today::run_two(input),
        now.elapsed().as_millis()
    );
}
