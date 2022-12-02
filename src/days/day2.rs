use std::collections::HashMap;

use crate::input::lib::read_input;

const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;
const LOSS_SCORE: i32 = 0;

const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SCISSORS_SCORE: i32 = 3;

fn score_for_round_part1(opponent_selection: &str, your_selection: &str) -> i32 {
    let score = HashMap::from([("X", ROCK_SCORE), ("Y", PAPER_SCORE), ("Z", SCISSORS_SCORE)]);
    let round_score = score.get(your_selection).unwrap();

    if opponent_selection == "A" && your_selection == "X" {
        return round_score + DRAW_SCORE;
    } else if opponent_selection == "A" && your_selection == "Y" {
        return round_score + WIN_SCORE;
    } else if opponent_selection == "B" && your_selection == "Y" {
        return round_score + DRAW_SCORE;
    } else if opponent_selection == "B" && your_selection == "Z" {
        return round_score + WIN_SCORE;
    } else if opponent_selection == "C" && your_selection == "X" {
        return round_score + WIN_SCORE;
    } else if opponent_selection == "C" && your_selection == "Z" {
        return round_score + DRAW_SCORE;
    }

    // loss
    return round_score.clone();
}

fn score_for_round_part2(opponent_selection: &str, desired_outcome: &str) -> i32 {
    let score = HashMap::from([("X", LOSS_SCORE), ("Y", DRAW_SCORE), ("Z", WIN_SCORE)]);
    let selection_score =
        HashMap::from([("A", ROCK_SCORE), ("B", PAPER_SCORE), ("C", SCISSORS_SCORE)]);
    let round_score = score.get(desired_outcome).unwrap();

    if opponent_selection == "A" && desired_outcome == "X" {
        return selection_score.get("C").unwrap().clone();
    } else if opponent_selection == "A" && desired_outcome == "Z" {
        return round_score + selection_score.get("B").unwrap();
    } else if opponent_selection == "B" && desired_outcome == "X" {
        return selection_score.get("A").unwrap().clone();
    } else if opponent_selection == "B" && desired_outcome == "Z" {
        return round_score + selection_score.get("C").unwrap();
    } else if opponent_selection == "C" && desired_outcome == "X" {
        return selection_score.get("B").unwrap().clone();
    } else if opponent_selection == "C" && desired_outcome == "Z" {
        return round_score + selection_score.get("A").unwrap();
    }

    // tie
    return round_score + selection_score.get(opponent_selection).unwrap();
}

pub fn total_score_for_strategy_guide() {
    let problem_input = read_input("./src/input/files/day2.txt");
    let rounds = problem_input.split("\n").collect::<Vec<&str>>();
    let mut total_score_pt1 = 0;
    let mut total_score_pt2 = 0;

    for round in rounds {
        let round_selections = round.split(" ").collect::<Vec<&str>>();
        let opponent_selection = round_selections.get(0).unwrap();
        let desired_outcome = round_selections.get(1).unwrap();

        total_score_pt1 += score_for_round_part1(opponent_selection, desired_outcome);
        total_score_pt2 += score_for_round_part2(opponent_selection, desired_outcome);
    }

    println!("Day 2, problem 1: {}", total_score_pt1);
    println!("Day 2, problem 2: {}", total_score_pt2);
}
