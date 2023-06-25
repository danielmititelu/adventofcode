use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

pub fn print_day_one_answers() {
    let elf_calories_map = get_elf_calories_map("input/day_one_input.txt");
    print_first_problem_answers(&elf_calories_map);
    print_second_problem_answers(&elf_calories_map);
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

fn print_second_problem_answers(elf_calories_map: &HashMap<i32, i32>) {
    let mut calories: Vec<&i32> = elf_calories_map
        .iter()
        .map(|(_k, v)| v)
        .collect::<Vec<&i32>>();

    calories.sort_by(|a, b| b.cmp(a));

    let top_three_sum: i32 = calories.iter().take(3).copied().sum();

    println!("Calories sum of top three elfs: {}", top_three_sum);
}

fn get_elf_calories_map(file_path: &str) -> HashMap<i32, i32> {
    let mut elf_calories_map: HashMap<i32, i32> = HashMap::new();
    let mut index = 0;

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file, '{}': {}", file_path, e);
            exit(1)
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                println!("Error reading line, {}", e);
                exit(1);
            }
        };

        if line == "" {
            index += 1;
            continue;
        }

        let elf_calories = match line.parse::<i32>() {
            Ok(calories) => calories,
            Err(e) => {
                println!("Could not convert '{}' into int32, {}", line, e);
                exit(1)
            }
        };

        match elf_calories_map.get(&index) {
            Some(current_elf_calories) => {
                elf_calories_map.insert(index, current_elf_calories + elf_calories);
            }
            None => {
                elf_calories_map.insert(index, elf_calories);
            }
        }
    }
    elf_calories_map
}
