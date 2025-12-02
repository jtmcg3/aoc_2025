
fn main() {
    let mut position = 50;
    let directions = read_file();
    let mut solution = 0;
    let mut boop = false;
    for direction in &directions {
        boop = false;
        if position == 0 {
            boop = true;
        }
        let steps = parse_direction(&direction);
        solution += steps.abs()/100;
        let offset = steps%100;
        
        println!("Direction: {} Starting Position: {}", direction, position);
        position = position + offset;
        
        if position < 0 {
            position = 100 + position;
            if !boop {
                solution += 1;
            }
        } else if position > 99 {
            position = position - 100;
            if !boop {
                solution += 1;
            }
        } else if position == 0 && !boop {
            solution += 1;
        }
        println!("Position {} Solution {}", position, solution)
    }
    println!("Solution: {}", solution);
    println!("Directions: {}", directions.len());
}

fn read_file() -> Vec<String> {
    let file_path = "src/secret/combo.txt";
    let contents = std::fs::read_to_string(file_path)
        .expect(&format!("Should have been able to read the file: {}", file_path));
    
    contents.lines().map(|s| s.to_string()).collect()
}

fn parse_direction(line: &str) -> i32 {
    let direction = &line[0..1];
    let steps = &line[1..];
    match direction {
        "L" => -1 * steps.parse::<i32>().unwrap(),
        "R" => 1 * steps.parse::<i32>().unwrap(),
        _ => panic!("Invalid direction: {}", direction),
    }
}
