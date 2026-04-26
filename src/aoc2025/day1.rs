pub fn solve(input: &str) {
    let mut password1 = 0;
    let mut password2 = 0;
    let mut start = 50;

    for line in input.lines() {
        let direction: &str = &line[0..1];
        let clicks: i32 = line[1..]
            .parse()
            .expect(&format!("\n\x1b[91mIvalid line {line}\n\n\x1b[0m"));

        // for part two
        if direction == "L" {
            let dist_l = if start == 0 { 100 } else { start };

            if clicks >= dist_l {
                password2 += 1 + (clicks - dist_l) / 100;
            }

            start = (start - (clicks % 100) + 100) % 100;
        } else {
            let dist_r = if start == 0 { 100 } else { 100 - start };

            if clicks >= dist_r {
                password2 += 1 + (clicks - dist_r) / 100;
            }

            start = (start + clicks) % 100;
        }

        //for part one
        if start == 0 {
            password1 = password1 + 1;
        }
    }
    print!("\n\naoc2025_day1 part-1 : {password1}\naoc2025_day1 part-2 : {password2}\n\n");
}
