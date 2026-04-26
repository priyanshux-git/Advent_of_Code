pub fn solve(input: &str) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    
    // print_grid(&grid);

    let mut part1 = 0u16;
    let mut part2 = 0u16;

    let rows = grid.len();
    let cols = grid.get(0).unwrap().len();

    // part-1
    for i in 0..rows {
        for j in 0..cols {
            if grid.get(i).unwrap().get(j).unwrap() == &'@' {
                let mut nb = 0u8;
                // check : [i,j+1], [i,j-1], [i+1,j], [i-1,j], [i+1,j+1], [i-1,j-1], [i+1, j-1], [i-1, j+1]
                if j < cols - 1 && grid.get(i).unwrap().get(j + 1).unwrap() == &'@' {
                    nb += 1;
                }

                if j > 0 && grid.get(i).unwrap().get(j - 1).unwrap() == &'@' {
                    nb += 1;
                }

                if i < rows - 1 && grid.get(i + 1).unwrap().get(j).unwrap() == &'@' {
                    nb += 1;
                }

                if i > 0 && grid.get(i - 1).unwrap().get(j).unwrap() == &'@' {
                    nb += 1;
                }

                if j < cols - 1
                    && i < rows - 1
                    && grid.get(i + 1).unwrap().get(j + 1).unwrap() == &'@'
                {
                    nb += 1;
                }

                if j > 0 && i > 0 && grid.get(i - 1).unwrap().get(j - 1).unwrap() == &'@' {
                    nb += 1;
                }

                if j > 0 && i < rows - 1 && grid.get(i + 1).unwrap().get(j - 1).unwrap() == &'@' {
                    nb += 1;
                }

                if j < cols - 1 && i > 0 && grid.get(i - 1).unwrap().get(j + 1).unwrap() == &'@' {
                    nb += 1;
                }

                if nb < 4 {
                    part1 += 1;
                }
            }
        }
    }

    // part 2
    let mut flag = true;

    while flag {
        flag = false;
        let mut rm: Vec<(usize, usize)> = vec![];

        for i in 0..rows {
            for j in 0..cols {
                if grid.get(i).unwrap().get(j).unwrap() == &'@' {
                    let mut nb = 0u8;
                    // check : [i,j+1], [i,j-1], [i+1,j], [i-1,j], [i+1,j+1], [i-1,j-1], [i+1, j-1], [i-1, j+1]
                    if j < cols - 1 && grid.get(i).unwrap().get(j + 1).unwrap() == &'@' {
                        nb += 1;
                    }

                    if j > 0 && grid.get(i).unwrap().get(j - 1).unwrap() == &'@' {
                        nb += 1;
                    }

                    if i < rows - 1 && grid.get(i + 1).unwrap().get(j).unwrap() == &'@' {
                        nb += 1;
                    }

                    if i > 0 && grid.get(i - 1).unwrap().get(j).unwrap() == &'@' {
                        nb += 1;
                    }

                    if j < cols - 1
                        && i < rows - 1
                        && grid.get(i + 1).unwrap().get(j + 1).unwrap() == &'@'
                    {
                        nb += 1;
                    }

                    if j > 0 && i > 0 && grid.get(i - 1).unwrap().get(j - 1).unwrap() == &'@' {
                        nb += 1;
                    }

                    if j > 0 && i < rows - 1 && grid.get(i + 1).unwrap().get(j - 1).unwrap() == &'@'
                    {
                        nb += 1;
                    }

                    if j < cols - 1 && i > 0 && grid.get(i - 1).unwrap().get(j + 1).unwrap() == &'@'
                    {
                        nb += 1;
                    }

                    if nb < 4 {
                        part2 += 1;
                        rm.push((i, j));
                    }
                }
            }
        }
        if !rm.is_empty() {
            flag = true;

            for (i, j) in rm {
                *grid.get_mut(i).unwrap().get_mut(j).unwrap() = '.';
            }
        }
    }
    // print_grid(&grid);
    print!("\naoc2025_day4 part-1 : {part1}\naoc2025_day4 part-2 : {part2}\n");
}

fn _print_grid(g: &Vec<Vec<char>>){
    for row in g{
        for col in row{
            print!("{col}");
        }
        println!();
    }
    println!();
}