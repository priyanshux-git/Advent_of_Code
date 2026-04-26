mod aoc2025;
mod input;

use input::get_input;

fn main() {
    let start_time = std::time::Instant::now();
    
    aoc2025::day1::solve(&get_input(2025, 1));
    aoc2025::day2::solve(&get_input(2025, 2));
    aoc2025::day3::solve(&get_input(2025, 3));
    
    println!("\nRuntime : {:?}\n", start_time.elapsed());
}
