pub fn solve(input: &str) {
    let mut invalid_sum1 = 0i64;
    let mut invalid_sum2 = 0i64;

    for range in input.split(",") {
        let mut rng = range.trim().split("-").map(|x| x.parse::<i64>().unwrap());
        let start = rng.next().unwrap();
        let end = rng.next().unwrap();

        for id in start..=end {
            let mut i = id;
            let mut length = 0;
            
            while i > 0 {
                i /= 10;
                length += 1;
            }
            
            for l in 1..=length / 2 {
                if length % l == 0 {
                    let modulo = (10f32).powi(l) as i64;
                    i = id;
                    let mut pre = i % modulo;
                    
                    while i > 0 {
                        if i % modulo == pre {
                            pre = i % modulo;
                            i /= modulo;
                            
                        } else {
                            break;
                        }
                    }
                    
                    if i == 0 {
                        invalid_sum2 += id;
                        break;
                    }
                }
            }
            
            if length % 2 == 1 {
                continue;
            }
            
            i = id;
            let mut place_value = 1;
            
            for _ in 0..length / 2 {
                i = i / 10;
                place_value *= 10;
            }

            if id % place_value == id / place_value {
                invalid_sum1 += id;
            }
        }
    }
    print!("\naoc2025_day2 part-1 : {invalid_sum1}\naoc2025_day2 part-2 : {invalid_sum2}\n");
}

