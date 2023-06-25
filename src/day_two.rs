use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

#[derive(PartialEq)]
enum HandChoice {
    Rock,
    Paper,
    Scissors,
}

enum DuelOutcome {
    Win,
    Lose,
    Tie,
}

pub fn print_day_two_answers() {
    let file = match File::open("input/day_two_input.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            exit(1)
        }
    };

    let reader = BufReader::new(file);

    let mut total_score = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => total_score += calculate_score(line),
            Err(e) => {
                println!("Error reading line, {}", e);
                exit(1);
            }
        }
    }

    println!("Total score: {}", total_score);
}

fn calculate_score(line: String) -> i32 {
    let letters: Vec<&str> = line.split(" ").collect();
    let my_letter = match letters.get(1) {
        Some(letter) => letter,
        None => {
            println!("Could not get my letter");
            exit(1);
        }
    };

    let opponent_letter = match letters.get(0) {
        Some(letter) => letter,
        None => {
            println!("Could not get opponent letter");
            exit(1);
        }
    };

    let my_choice = map_letter_to_hand_choice(my_letter);
    let opponent_choice = map_letter_to_hand_choice(opponent_letter);

    let hand_score = match my_choice {
        HandChoice::Rock => 1,
        HandChoice::Paper => 2,
        HandChoice::Scissors => 3,
    };
    
    let duel_outcome = calculate_duel_outcome(&my_choice, &opponent_choice);
    match duel_outcome {
        DuelOutcome::Win => 6 + hand_score,
        DuelOutcome::Tie => 3 + hand_score,
        DuelOutcome::Lose => hand_score,
    }
}

fn calculate_duel_outcome(my_choice: &HandChoice, opponent_choice: &HandChoice) -> DuelOutcome {
    if my_choice == opponent_choice {
        return DuelOutcome::Tie;
    }

    if (my_choice == &HandChoice::Rock && opponent_choice == &HandChoice::Scissors)
        || (my_choice == &HandChoice::Paper && opponent_choice == &HandChoice::Rock)
        || (my_choice == &HandChoice::Scissors && opponent_choice == &HandChoice::Paper)
    {
        return DuelOutcome::Win;
    }

    return DuelOutcome::Lose;
}

fn map_letter_to_hand_choice(letter: &str) -> HandChoice {
    let hand_choice = match letter {
        "A" | "X" => HandChoice::Rock,
        "B" | "Y" => HandChoice::Paper,
        "C" | "Z" => HandChoice::Scissors,
        _ => {
            println!("Invalid letter: {}", letter);
            exit(1)
        }
    };
    hand_choice
}
