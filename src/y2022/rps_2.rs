use std::fs;
use std::io::{BufReader, Lines};

#[derive(Clone)]
enum RpsChoice {
    Rock,
    Paper,
    Scissor
}
enum RpsOutcome {
    Win,
    Draw,
    Loss
}
fn calc_rps_choice_score(choice: &RpsChoice) -> i32 {
    match choice {
        RpsChoice::Rock => 1,
        RpsChoice::Paper => 2,
        RpsChoice::Scissor => 3
    }
}fn calc_rps_outcome_score(choice: &RpsOutcome) -> i32 {
    match choice {
        RpsOutcome::Win => 6,
        RpsOutcome::Draw => 3,
        RpsOutcome::Loss => 0
    }
}

fn calc_rps_game(player_choice : &RpsChoice, opponent_choice: &RpsChoice) -> RpsOutcome {
    match player_choice {
        RpsChoice::Rock => match opponent_choice { 
            RpsChoice::Rock => RpsOutcome::Draw,
            RpsChoice::Paper => RpsOutcome::Loss,
            RpsChoice::Scissor => RpsOutcome::Win
        },
        RpsChoice::Paper => match opponent_choice { 
            RpsChoice::Rock => RpsOutcome::Win,
            RpsChoice::Paper => RpsOutcome::Draw,
            RpsChoice::Scissor => RpsOutcome::Loss
        },
        RpsChoice::Scissor => match opponent_choice { 
            RpsChoice::Rock => RpsOutcome::Loss,
            RpsChoice::Paper => RpsOutcome::Win,
            RpsChoice::Scissor => RpsOutcome::Draw
        }
    }
}

fn reverse_calc_rps_game(outcome : &RpsOutcome, opponent_choice: &RpsChoice) -> RpsChoice {
    match outcome {
        RpsOutcome::Win => match opponent_choice { 
            RpsChoice::Rock => RpsChoice::Paper,
            RpsChoice::Paper => RpsChoice::Scissor,
            RpsChoice::Scissor => RpsChoice::Rock
        },
        RpsOutcome::Draw => opponent_choice.clone(),
        RpsOutcome::Loss => match opponent_choice { 
            RpsChoice::Rock => RpsChoice::Scissor,
            RpsChoice::Paper => RpsChoice::Rock,
            RpsChoice::Scissor => RpsChoice::Paper
        }
    }
}

pub fn rps_a(lines : Lines<BufReader<fs::File>>) {
    let mut score = 0;
    for line in lines.map(|x| x.unwrap()) {
        if line.len() != 3 {
            continue;
        }
        let opponent_choice = match line.chars().nth(0).expect("Error parsing first column") {
            'A' => RpsChoice::Rock,
            'B' => RpsChoice::Paper,
            'C' => RpsChoice::Scissor,
            _ => panic!("Error parsing first column")
        };
        let player_choice = match line.chars().nth(2).expect("Error parsing second column") {
            'X' => RpsChoice::Rock,
            'Y' => RpsChoice::Paper,
            'Z' => RpsChoice::Scissor,
            _ => panic!("Error parsing second column")
        };
        let outcome = calc_rps_game(&player_choice, &opponent_choice);
        score = score + calc_rps_outcome_score(&outcome) + calc_rps_choice_score(&player_choice)
    }
    println!("{}", score);
}

pub fn rps_b(lines : Lines<BufReader<fs::File>>) {
    let mut score = 0;
    for line in lines.map(|x| x.unwrap()) {
        if line.len() != 3 {
            continue;
        }
        let opponent_choice = match line.chars().nth(0).expect("Error parsing first column") {
            'A' => RpsChoice::Rock,
            'B' => RpsChoice::Paper,
            'C' => RpsChoice::Scissor,
            _ => panic!("Error parsing first column")
        };
        let outcome = match line.chars().nth(2).expect("Error parsing second column") {
            'X' => RpsOutcome::Loss,
            'Y' => RpsOutcome::Draw,
            'Z' => RpsOutcome::Win,
            _ => panic!("Error parsing second column")
        };
        let player_choice = reverse_calc_rps_game(&outcome, &opponent_choice);
        score = score + calc_rps_outcome_score(&outcome) + calc_rps_choice_score(&player_choice)
    }
    println!("{}", score);
}


