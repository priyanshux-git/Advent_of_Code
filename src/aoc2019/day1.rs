pub fn solve(input: &str) {
    let mut part1 = 0;
    let mut part2 = 0;

    for ms in input.lines() {
        let mut mass: i32 = ms.trim().parse().unwrap();
        
        // part-1
        part1 += (mass / 3) - 2;
        
        // part-2
        while mass > 0 {
            mass = (mass / 3) - 2;
            part2 += if mass > 0 { mass } else { 0 };
        }
    }

    print!("\naoc2019_day1 part-1 : {part1}\naoc2019_day1 part-2 : {part2}\n");
}