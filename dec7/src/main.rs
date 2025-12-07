fn main() {
    let mut graph = read_file("src/input/input.txt");
    //print_problem(&graph);
    let start = find_start(&graph[0]);
    println!("Start: {}", start);
    let count = many_worlds_backtracker(&mut graph, start, 1);
    println!("Count: {}", count);
    //print_problem(&graph);
}


fn read_file(input: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(input).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

fn print_problem(graph: &Vec<String>) {
    for line in graph {
        println!("{}", line);
    }
}

fn find_start(top_row: &String) -> usize {
    for (i, c) in top_row.chars().enumerate() {
        if c == 'S' {
            return i;
        }
    }
    panic!("No start found");
}



fn recursive_beam(graph: &mut Vec<String>, col: usize, row: usize) -> i64 {
    let mut count: i64 = 0;
   //println!("({}, {})", col, row);
    if row < graph.len()-1 {
        match graph[row].chars().nth(col).unwrap() {
            '.' => {
                graph[row].replace_range(col..col+1, "|");
                count += recursive_beam(graph, col, row+1);
            },
            '^' => {
                //println!("Splitting Beam at({}, {})", col, row);
                count += 1;
                if graph[row].chars().nth(col+1).unwrap() == '.' {
                    graph[row].replace_range(col+1..col+2, "|");
                    count +=recursive_beam(graph, col+1, row+1);
                }
                if graph[row].chars().nth(col-1).unwrap() == '.' {
                    graph[row].replace_range(col-1..col, "|");
                    count +=recursive_beam(graph, col-1, row+1);
                }
            },
            '|' => {
                count +=0;
            },
            _ => {
                panic!("OH-NO!")
            }
        }   
    }
    count
}

            
// i believe the solution for part 2 is by using backtracking

use std::collections::HashMap;

fn many_worlds_backtracker(graph: &Vec<String>, col: usize, row: usize) -> i64 {
    let mut cache: HashMap<(usize, usize), i64> = HashMap::new();

    fn backtracking(graph: &Vec<String>, col: usize, row: usize, cache: &mut HashMap<(usize, usize), i64>) -> i64 {
        if let Some(&count) = cache.get(&(col, row)) {
            return count;
        }

        if row >= graph.len() - 1 {
            return 1;
        }

        let mut count = 0;
        match graph[row].chars().nth(col).unwrap() {
            '.' => {
                count += backtracking(graph, col, row + 1, cache);
            },
            '^' => {
                count += backtracking(graph, col + 1, row + 1, cache);
                count += backtracking(graph, col - 1, row + 1, cache);
            },
            _ => panic!("OH-NO!")
        }
        
        cache.insert((col, row), count);
        count
    }

    backtracking(graph, col, row, &mut cache)
}