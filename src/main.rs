mod aoc2025;
mod input;

use input::get_input;
fn main() {
    let start_time = std::time::Instant::now();
    
    aoc2025::day1::solve(&get_input(2025, 1));
    
    println!("Runtime : {:?}\n", start_time.elapsed());
}
