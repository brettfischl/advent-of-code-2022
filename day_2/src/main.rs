use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead, BufReader, Lines};
use std::fs::File;
use std::path::Path;

#[derive(PartialEq, Eq, Hash)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Hash)]
enum Outcome {
    Win,
    Lose,
    Tie
}


fn choose_winner(choice_one: &Choice, choice_two: &Choice) -> &'static str {
    if choice_one == choice_two {
        return "tie"
    }


    if choice_one == &Choice::Rock && choice_two == &Choice::Paper {
        return "player_two"
    }

    if choice_one == &Choice::Paper && choice_two == &Choice::Scissors {
        return "player_two"
    }

    if choice_one == &Choice::Scissors && choice_two == &Choice::Rock {
        return "player_two"
    }


    return "player_one"
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("error: not enough args");
        return;
    }
    let file_path = &args[1];
    let question_part = &args[2];

    println!("question part: {}", question_part);
    println!("finding file: {}", file_path);

    if let Ok(lines) = read_lines(file_path) {
        if question_part == "1" {
            println!("your score is {}", part_one(lines));
        } else if question_part == "2" {
            println!("your score is {}", part_two(lines));
        }
    }
}

fn part_two(lines: Lines<BufReader<File>>) -> i32 {
    let mut score = 0;

    let score_map = HashMap::from([
        (Choice::Rock, 1),
        (Choice::Paper, 2),
        (Choice::Scissors, 3)
    ]);

    let code_map = HashMap::from([
        ("A", Choice::Rock),
        ("B", Choice::Paper),
        ("C", Choice::Scissors)
    ]);

    let outcome_map = HashMap::from([
        ("X", Outcome::Lose),
        ("Y", Outcome::Tie),
        ("Z", Outcome::Win)
    ]);

    let loss_map = HashMap::from([
        (Choice::Rock, Choice::Scissors),
        (Choice::Scissors, Choice::Paper),
        (Choice::Paper, Choice::Rock)
    ]);

    let win_map = HashMap::from([
        (Choice::Rock, Choice::Paper),
        (Choice::Scissors, Choice::Rock),
        (Choice::Paper, Choice::Scissors)
    ]);

    for line in lines {
        if let Ok(ln) = line {
            let codes: Vec<&str> = ln.split(" ").collect();
            let opp_choice = code_map.get(codes[0]).unwrap();
            let player_outcome = outcome_map.get(codes[1]).unwrap();

            if player_outcome == &Outcome::Lose {
                score += score_map.get(loss_map.get(opp_choice).unwrap()).unwrap();
            } else if player_outcome == &Outcome::Win {
                score += score_map.get(win_map.get(opp_choice).unwrap()).unwrap();
                score += 6
            } else {
                score += score_map.get(opp_choice).unwrap();
                score += 3
            }
        }
    }

    return score;
}

fn part_one(lines: Lines<BufReader<File>>) -> i32 {

    let code_map = HashMap::from([
        ("A", Choice::Rock),
        ("B", Choice::Paper),
        ("C", Choice::Scissors),
        ("X", Choice::Rock),
        ("Y", Choice::Paper),
        ("Z", Choice::Scissors)
    ]);

    let score_map = HashMap::from([
        (Choice::Rock, 1),
        (Choice::Paper, 2),
        (Choice::Scissors, 3)
    ]);

    let mut score = 0;

    for line in lines {
        if let Ok(ln) = line {
            let codes: Vec<&str> = ln.split(" ").collect();
            let opp_choice = code_map.get(codes[0]).unwrap();
            let player_choice = code_map.get(codes[1]).unwrap();
            let winner = choose_winner(opp_choice, player_choice);

            score += score_map.get(player_choice).unwrap();

            if winner == "player_two" {
                score += 6
            }

            if winner == "tie" {
                score += 3
            }

        }
    }
    return score;

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
