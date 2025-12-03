fn main() {
    let input = read_file("src/input/input.txt");
    let mut solution: u64 = 0;
    for line in input {
        solution += max_voltage(&line);
    }
    println!("Solution: {}", solution);
}


fn read_file(input: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(input).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

/* it's always the max in range 0..length of string-1 + 
   the max in the range from that max index to the end of string 
   need to convert str to u8 to compare
   */

fn max_first_digit(input: &str) -> usize {
    let mut max_idx = 0;

    for (idx, c) in input.chars().enumerate() {
        if idx == input.len()-11 { //part 2 is 12 digits
            break;
        } else if c as u8 > input.chars().nth(max_idx).unwrap() as u8 {
            max_idx = idx;
        }
    }
    println!("First Digit's index: {}, First Digit {}", max_idx, input.chars().nth(max_idx).unwrap());

    max_idx
}

fn max_second_digit(input: &str, first_max_index: usize) -> usize {
    let mut max_idx = first_max_index+1;

    for (idx, c) in input.chars().enumerate() {
        if idx < first_max_index+1 {
            continue;
        } else if idx > input.len()-1 {
            break;
        } else if c as u8 > input.chars().nth(max_idx).unwrap() as u8 {
            max_idx = idx;
        }
    }
    //println!("Second Digit's index: {}, Second Digit {}", max_idx, input.chars().nth(max_idx).unwrap());
    max_idx
}

fn max_next_digit(input: &str, first_max_index: usize, end_range: usize) -> usize {
    let mut max_idx = first_max_index+1;

    for (idx, c) in input.chars().enumerate() {
        if idx < first_max_index+1 {
            continue;
        } else if idx > end_range {
            break;
        } else if c as u8 > input.chars().nth(max_idx).unwrap() as u8 {
            max_idx = idx;
        }
    }
    //println!("Next Digit's index: {}, Next Digit {}", max_idx, input.chars().nth(max_idx).unwrap());
    max_idx
}

// using u64 because the sum can get large
fn max_voltage(input: &str) -> u64 {
    let first_max_index = max_first_digit(input);
    let mut next_max_index = first_max_index;
    let mut output: String = input.chars().nth(first_max_index).unwrap().to_string();

    if first_max_index == input.len()-11 { //part 2 is 12 digits
        return input[first_max_index..].parse::<u64>().unwrap();
    } else {
        for _ in 0..11 {
            let end_range = input.len()-(12-output.len());
            next_max_index = max_next_digit(input, next_max_index, end_range);
            output = format!("{}{}", output, input.chars().nth(next_max_index).unwrap());
        }
        println!("Output: {} from Input: {}", output, input);
        return output.parse::<u64>().unwrap();
    }
}

// part 2 is 12 digits, yay.
