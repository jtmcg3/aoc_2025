fn main() {
    let input = read_file("src/input/input.txt");
    let list = list_from_str(&input[0]);
    let tuples = list_of_tuples_from_list_of_strings(&list);
    let mut solution: u64 = 0;
    for tuple in &tuples {
        for integer in tuple.0..=tuple.1 {
            // check if integer is invalid;
            if is_invalid(integer) {
                solution += integer;
                println!("{} is invalid", integer);
            }
        }
    }
    println!("Solution: {}", solution);
}

fn read_file(input: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(input).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

fn list_from_str(input: &str) -> Vec<String> {
    input.split(",").map(|s| s.to_string()).collect()
}

fn tuples_from_str(input: &str) -> (u64, u64) {
    let list: Vec<u64> = input.split("-").map(|s| s.parse::<u64>().unwrap()).collect();
    (list[0], list[1])
}

fn list_of_tuples_from_list_of_strings(input: &Vec<String>) -> Vec<(u64, u64)> {
    input.iter().map(|s| tuples_from_str(s)).collect()
}

fn is_invalid(input: u64) -> bool {
    // convert to string for easier parsing
    let input_str = input.to_string();
    if input_str.len() % 2 == 0 {
        let span = input_str.len() / 2;
        let first = &input_str[0..span];
        let second = &input_str[span..];
        if first == second {
            return true;
        }
    }
    false
}
