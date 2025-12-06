fn main() {
    let input = read_file("src/input/input.txt");
    let (number_matrix, symbol_vec) = parse_input(&input);
    print_problem(&number_matrix);
    println!("Symbols: {:?}", symbol_vec);
    let mut solution: i64 = 0;
    for i in 0..symbol_vec.len() {
        solution += operate_problem(&number_matrix, symbol_vec.clone(), i);
    }
    println!("Solution: {}", solution);
}

fn read_file(input: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(input).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

fn print_problem(input: &Vec<Vec<i64>>) {
    for line in input {
        println!("{:?}", line);
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<Vec<i64>>, Vec<String>){
    let mut number_matrix: Vec<Vec<i64>> = Vec::new();
    let mut symbol_vec: Vec<String> = Vec::new();
    
    let mut problem_number: usize = 0;
    number_matrix.push(Vec::new());

    for i in (0..input[0].len()).rev() {
        
        let next_number = format!("{}{}{}{}",
                                  input[0].chars().nth(i).unwrap(),
                                  input[1].chars().nth(i).unwrap(),
                                  input[2].chars().nth(i).unwrap(),
                                  input[3].chars().nth(i).unwrap())
                                  .trim().to_string();

        let parsed_number = next_number.parse::<i64>().unwrap_or(-1);
        if parsed_number == -1 {
            continue;
        } else {
            number_matrix[problem_number].push(parsed_number);
        }

        if input[4].chars().nth(i).unwrap() == '+' || input[4].chars().nth(i).unwrap() == '*' {
            symbol_vec.push(input[4].chars().nth(i).unwrap().to_string());
            problem_number += 1;
            number_matrix.push(Vec::new())
        } 
        
    }

    (number_matrix, symbol_vec)
}

fn operate_column(input: &Vec<Vec<i64>>, symbol: &str, column: usize) -> i64 {
    let mut output: i64 = input[0][column];
    for row in input.iter().skip(1) {
        match symbol.chars().nth(column).unwrap() {
            '+' => output += row[column],
            '*' => output *= row[column],
            _ => {}
        }
    }
    output
}

fn operate_problem(input: &Vec<Vec<i64>>, symbol: Vec<String>, problem_number: usize) -> i64 {
    let mut output: i64 = 0;
    if input[problem_number].len() != 0 {
        output = input[problem_number][0];
        for num in input[problem_number].iter().skip(1) {
            match symbol[problem_number].chars().nth(0).unwrap() {
                '+' => output += num,
                '*' => output *= num,
                _ => {}
            }
        }
    }
    println!("Output: {} Problem Number: {}", output, problem_number);
    output
}