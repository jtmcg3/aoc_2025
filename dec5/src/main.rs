use std::collections::HashSet;

fn main() {
    let input = read_file("src/input/input.txt");
    let (mut fresh_ranges, ingredients) = parse_file(&input);
    //sort ranges by starting, store as tuples or list of lists
    fresh_ranges.sort_by_key(|range| range.0);
    //println!("Fresh ranges: {:?}", fresh_ranges);
    //println!("Ingredients: {:?}", ingredients);

    // set to track fresh ingredients
    let mut fresh_ingredients = HashSet::new();

    // iterate l to r
    // if start greater than ingredient, break
    // if greater than start
    //.   if ingredient less than or equal to end, add to fresh set
    //.   if ingredient greater than end, continue
    for ingredient in ingredients {
        for range in &fresh_ranges {
            if range.0 > ingredient {
                break;
            } else if range.1 < ingredient {
                continue;
            } else if range.0 <= ingredient && ingredient <= range.1 {
                fresh_ingredients.insert(ingredient);
                break;
            } else {
                continue;
            }
        }
    }
    println!("Fresh ingredients: {:?}", fresh_ingredients);
    println!("Number of fresh ingredients: {}", fresh_ingredients.len());
}

fn read_file(input: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(input).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

fn parse_file(input: &Vec<String>) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut fresh_ranges = Vec::new();
    let mut ingredients = Vec::new();
    for line in input {
        if line.is_empty() {
            continue;
        }
        if line.contains("-") {
            let (start, end) = line.split_once("-").unwrap();
            fresh_ranges.push((start.parse().unwrap(), end.parse().unwrap()));
        } else {
            ingredients.push(line.parse().unwrap());
        }
    }
    (fresh_ranges, ingredients)
}


// use a set to track fresh ingredients

// sort ranges by starting, store as tuples or list of lists

// iterate l to r
// if start greater than ingredient, break
// if greater than start
//.   if ingredient less than or equal to end, add to fresh set
//.   if ingredient greater than end, continue


/* 
--- Day 5: Cafeteria ---

As the forklifts break through the wall, the Elves are delighted to discover that there was a cafeteria on the other side after all.

You can hear a commotion coming from the kitchen. "At this rate, we won't have any time left to put the wreaths up in the dining hall!" Resolute in your quest, you investigate.

"If only we hadn't switched to the new inventory management system right before Christmas!" another Elf exclaims. You ask what's going on.

The Elves in the kitchen explain the situation: because of their complicated new inventory management system, they can't figure out which of their ingredients are fresh and which are spoiled. When you ask how it works, they give you a copy of their database (your puzzle input).

The database operates on ingredient IDs. It consists of a list of fresh ingredient ID ranges, a blank line, and a list of available ingredient IDs. For example:

3-5
10-14
16-20
12-18

1
5
8
11
17
32
The fresh ID ranges are inclusive: the range 3-5 means that ingredient IDs 3, 4, and 5 are all fresh. The ranges can also overlap; an ingredient ID is fresh if it is in any range.

The Elves are trying to determine which of the available ingredient IDs are fresh. In this example, this is done as follows:

Ingredient ID 1 is spoiled because it does not fall into any range.
Ingredient ID 5 is fresh because it falls into range 3-5.
Ingredient ID 8 is spoiled.
Ingredient ID 11 is fresh because it falls into range 10-14.
Ingredient ID 17 is fresh because it falls into range 16-20 as well as range 12-18.
Ingredient ID 32 is spoiled.
So, in this example, 3 of the available ingredient IDs are fresh.

Process the database file from the new inventory management system. How many of the available ingredient IDs are fresh?
*/