pub fn solve(input: &str) {
    let mut lines = input.lines();

    let mut fresh_range: Vec<(u64, u64)> = Vec::new();

    while let Some(ln) = lines.next()
        && ln != ""
    {
        let mut range = ln.split("-").map(|l| l.parse::<u64>().unwrap());
        fresh_range.push((range.next().unwrap(), range.next().unwrap()));
    }

    let mut fersh_count_part1 = 0_u64;
    let mut fresh_count_part2 = 0_u64;

    // part-1
    for ln in lines {
        let id = ln.parse::<u64>().unwrap();

        for range in &fresh_range {
            if id >= range.0 && id <= range.1 {
                fersh_count_part1 += 1;
                break;
            }
        }
    }

    // part-2
    fresh_range.sort();

    let mut mx = 0_u64;

    for i in 0..fresh_range.len() {
        let curr = fresh_range.get(i).unwrap();

        if curr.0 > mx {
            fresh_count_part2 += curr.1 - curr.0 + 1;
            mx = curr.1;
        } else if curr.1 > mx {
            fresh_count_part2 += curr.1 - mx;
            mx =  curr.1;
            
        }
    }

    // println!("{:?}",fresh_range);

    print!(
        "\naoc2025_day5 part-1 : {fersh_count_part1}\naoc2025_day5 part-2 : {fresh_count_part2}\n"
    );
}
