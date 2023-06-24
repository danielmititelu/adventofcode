use std::collections::HashMap;
use std::fs;
use std::process::exit;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let elf_calories_map = get_elf_calories_map(contents);
    println!("# First problem: find the elf carring the most calories");
    print_first_problem_answers(&elf_calories_map);

    println!("# Second problem: find the top three Elves carrying the most Calories and print the sum of their calories");
    print_second_problem_answers(&elf_calories_map);
}

fn print_second_problem_answers(elf_calories_map: &HashMap<i32, i32>) {
    let mut calories: Vec<&i32> = elf_calories_map
        .iter()
        .map(|(_k, v)| v)
        .collect::<Vec<&i32>>();

    calories.sort_by(|a, b| b.cmp(a));

    let top_three_sum: i32 = calories
        .iter()
        .take(3)
        .copied()
        .sum();

    println!("Sum of top three: {}", top_three_sum);
}

fn print_first_problem_answers(elf_calories_map: &HashMap<i32, i32>) {
    let max_calories_elf = elf_calories_map
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(_k, v)| v);

    match max_calories_elf {
        Some(v) => println!("Max calories elf: {}", v),
        None => println!("No max calories elf"),
    }
}

fn get_elf_calories_map(contents: String) -> HashMap<i32, i32> {
    let splited_contents = contents.split("\n");
    let mut elf_calories_map: HashMap<i32, i32> = HashMap::new();
    let mut index = 0;

    for line in splited_contents {
        if line == "" {
            index += 1;
            continue;
        }

        let calories = line.parse::<i32>();
        match calories {
            Ok(calories) => {
                let elf_calories = elf_calories_map.get(&index);
                match elf_calories {
                    Some(v) => {
                        elf_calories_map.insert(index, v + calories);
                    }
                    None => {
                        elf_calories_map.insert(index, calories);
                    },
                }
            }
            Err(_e) => {
                println!("Error: {}, {}", _e, line);
                exit(1)
            }
        }
    }
    elf_calories_map
}
