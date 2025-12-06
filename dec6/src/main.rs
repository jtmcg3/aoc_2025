fn main() {
    let input = read_file("src/input/test.txt");
    let (number_matrix, symbol_vec) = parse_input(&input);
    //print_problem(&number_matrix);
    //println!("Symbols: {:?}", symbol_vec);
    let mut solution: u64 = 0;
    for i in 0..symbol_vec[0].len() {
        solution += operate_column(&number_matrix, &symbol_vec[0], i);
    }
    println!("Solution: {}", solution);
}

fn read_file(input: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(input).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

fn print_problem(input: &Vec<Vec<u64>>) {
    for line in input {
        println!("{:?}", line);
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<Vec<u64>>, Vec<String>){
    let mut number_matrix: Vec<Vec<u64>> = Vec::new();
    let mut symbol_vec: Vec<String> = Vec::new();
    for line in input {
        if line.chars().nth(0).unwrap() != '*' {
            let row: Vec<u64> = line.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
            number_matrix.push(row);
        } else {
            symbol_vec.push(line.split_whitespace().collect());
        }
    }
    (number_matrix, symbol_vec)
}

fn operate_column(input: &Vec<Vec<u64>>, symbol: &str, column: usize) -> u64 {
    let mut output: u64 = input[0][column];
    for row in input.iter().skip(1) {
        match symbol.chars().nth(column).unwrap() {
            '+' => output += row[column],
            '*' => output *= row[column],
            _ => {}
        }
    }
    output
}