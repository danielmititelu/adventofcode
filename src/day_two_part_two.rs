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

#[derive(PartialEq)]
enum DuelOutcome {
    Win,
    Lose,
    Tie,
}

pub fn print_day_two_part_two_answers() {
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

    println!("Total score after correct requirments: {}", total_score);
}

fn calculate_score(line: String) -> i32 {
    let letters: Vec<&str> = line.split(" ").collect();

    let opponent_letter = match letters.get(0) {
        Some(letter) => letter,
        None => {
            println!("Could not get opponent letter");
            exit(1);
        }
    };

    let duel_outcome_letter = match letters.get(1) {
        Some(letter) => letter,
        None => {
            println!("Could not get my letter");
            exit(1);
        }
    };

    let opponent_choice = map_letter_to_hand_choice(&opponent_letter);
    let duel_outcome = map_letter_to_duel_outcome(duel_outcome_letter);
    let my_choice = calculate_my_choice(opponent_choice, &duel_outcome);

    let hand_score = match my_choice {
        HandChoice::Rock => 1,
        HandChoice::Paper => 2,
        HandChoice::Scissors => 3,
    };
    
    match duel_outcome {
        DuelOutcome::Win => 6 + hand_score,
        DuelOutcome::Tie => 3 + hand_score,
        DuelOutcome::Lose => hand_score,
    }
}

fn calculate_my_choice(opponent_choice: HandChoice, duel_outcome: &DuelOutcome) -> HandChoice {
    match duel_outcome {
        DuelOutcome::Win => match opponent_choice {
            HandChoice::Rock => HandChoice::Paper,
            HandChoice::Paper => HandChoice::Scissors,
            HandChoice::Scissors => HandChoice::Rock,
        }
        DuelOutcome::Lose => match opponent_choice {
            HandChoice::Rock => HandChoice::Scissors,
            HandChoice::Paper => HandChoice::Rock,
            HandChoice::Scissors => HandChoice::Paper,
        }
        DuelOutcome::Tie => opponent_choice,
    }
}

fn map_letter_to_duel_outcome(duel_outcome_letter: &str) -> DuelOutcome {
    match duel_outcome_letter {
        "X" => DuelOutcome::Lose,
        "Y" => DuelOutcome::Tie,
        "Z" => DuelOutcome::Win,
        _ => {
            println!("Invalid letter for duel outcome: {}", duel_outcome_letter);
            exit(1);
        }
    }
}

fn map_letter_to_hand_choice(letter: &str) -> HandChoice {
    let hand_choice = match letter {
        "A" => HandChoice::Rock,
        "B" => HandChoice::Paper,
        "C" => HandChoice::Scissors,
        _ => {
            println!("Invalid letter for hand choice: {}", letter);
            exit(1)
        }
    };
    hand_choice
}
