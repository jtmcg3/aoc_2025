fn main() {
    
    let mut input = read_file("src/input/input.txt");
    //print_grid(&input);

    let mut accessible_spots = 0;
    let mut prev_accessible_spots = -1;
    while accessible_spots != prev_accessible_spots {
        prev_accessible_spots = accessible_spots;
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if input[i].chars().nth(j).unwrap() == '@' {
                    let rolls = count_rolls_around_roll(&input, i, j);
                    if rolls < 4 {
                        mark_accessible_spots(&mut input, i, j);
                        accessible_spots += 1;
                    }
                }
            }
        }
        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if input[i].chars().nth(j).unwrap() == 'x' {
                    mark_removed_spots(&mut input, i, j);
                }
            }
        }
    }
    //print_grid(&input);
    println!("Accessible spots: {}", accessible_spots);
    
}

/* 
The forklifts can only access a roll of paper if 
there are fewer than four rolls of paper in the 
eight adjacent positions. 

rolls of paper = '@'
empty space = '.'
TODO: add
accessible roll of paper = 'x'
*/

fn read_file(input: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(input).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

// print the grid
fn print_grid(grid: &Vec<String>) {
    for line in grid {
        println!("{:?}", line);
    }
}

fn count_rolls_around_roll(grid: &Vec<String>, x: usize, y: usize) -> u64 {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            let check_x = x as isize + i;
            let check_y = y as isize + j;
            //println!("{} {}", check_x, check_y);
            if check_x < 0 || check_y < 0 || check_x >= grid.len() as isize || check_y >= grid[check_x as usize].len() as isize || (check_x == x as isize && check_y == y as isize) {
                continue;
            } else if grid[check_x as usize].chars().nth(check_y as usize).unwrap() == '@' {
                count += 1;
            } else if grid[check_x as usize].chars().nth(check_y as usize).unwrap() == 'x' {
                count += 1;
            }else {
                continue; // this would be a '.'
            }
        }
    }
    count
}

// mark x in accessible spots
fn mark_accessible_spots(grid: &mut Vec<String>, x: usize, y: usize) {
    grid[x].replace_range(y..y+1, "x");
}

fn mark_removed_spots(grid: &mut Vec<String>, x: usize, y: usize) {
    grid[x].replace_range(y..y+1, ".");
}
    