pub fn solve(input: &str) {
    let mut max_joltage1 = 0u16;
    let mut max_joltage2 = 0u64;

    for battery_bank in input.lines() {
        let mut pre_max: [u8; 2] = [0u8, 0u8];
        let mut max: [u8; 2] = [0u8, 0u8];
        let mut index: u8 = 0u8;
        let bank: &str = battery_bank.trim();
        let bank_arr: &[u8] = bank.as_bytes();
        let mut window_size = bank_arr.len() - 11usize;
        let mut curr_index = 0usize;
        let mut curr_num = 0u64;
        let mut done = 0usize;

        while done < 12usize {
            let mut max_num = [0u8, 0u8];

            for i in curr_index..curr_index + window_size {
                let num = bank_arr[i] - b'0';

                if num > max_num[0] {
                    max_num[0] = num;
                    max_num[1] = i as u8;
                }
            }

            curr_num *= 10u64;
            curr_num += max_num[0] as u64;
            done += 1usize;
            curr_index = (max_num[1] + 1) as usize;
            window_size = bank_arr.len() - (max_num[1] as usize) - (12usize - done);
        }

        max_joltage2 += curr_num;

        for b in bank.bytes() {
            let num: u8 = b - b'0';

            if num > max[0] {
                pre_max[0] = max[0];
                pre_max[1] = max[1];
                max[0] = num;
                max[1] = index;
            } else if num >= pre_max[0] {
                pre_max[0] = num;
                pre_max[1] = index;
            }

            index += 1;
        }

        if bank.len() - 1 == (max[1] as usize) {
            max_joltage1 += (max[0] as u16) + (pre_max[0] as u16) * 10u16;
        } else {
            if pre_max[1] > max[1] {
                max_joltage1 += (max[0] as u16) * 10u16 + (pre_max[0] as u16);
            } else {
                let mut new_max: u8 = 0u8;

                for i in (max[1] + 1u8) as usize..bank.len() {
                    let num: u8 = bank_arr[i] - b'0';

                    if num > new_max {
                        new_max = num;
                    }
                }

                max_joltage1 += (max[0] as u16) * 10u16 + (new_max as u16);
            }
        }
    }
    
    print!("\naoc2025_day3 part-1 : {max_joltage1}\naoc2025_day3 part-2 : {max_joltage2}\n");
}
