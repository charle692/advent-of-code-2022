use std::{collections::HashMap, str::FromStr};

use crate::input::lib::read_input;

// Horizontal moves
const RIGHT: &str = "R";
const LEFT: &str = "L";

// Vertical moves
const UP: &str = "U";
const DOWN: &str = "D";

// Diagonal moves
const UP_RIGHT: &str = "UR";
const UP_LEFT: &str = "UL";
const DOWN_RIGHT: &str = "DR";
const DOWN_LEFT: &str = "DL";

#[derive(Debug)]
struct Move {
    direction: String,
    amount: i32,
}

impl Move {
    pub fn new(direction: &str, amount: &str) -> Self {
        Move {
            direction: direction.to_string(),
            amount: <i32 as FromStr>::from_str(amount).unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_up(&mut self) {
        self.y += 1;
    }

    pub fn move_down(&mut self) {
        self.y -= 1;
    }

    pub fn move_up_right(&mut self) {
        self.x += 1;
        self.y += 1;
    }

    pub fn move_up_left(&mut self) {
        self.x -= 1;
        self.y += 1;
    }

    pub fn move_down_right(&mut self) {
        self.x += 1;
        self.y -= 1;
    }

    pub fn move_down_left(&mut self) {
        self.x -= 1;
        self.y -= 1;
    }
}

fn parse_input(problem_input: String) -> Vec<Move> {
    let moves_input = problem_input.split("\n").collect::<Vec<&str>>();

    moves_input.iter().fold(vec![], |mut moves, move_input| {
        let move_input_split = move_input.split(" ").collect::<Vec<&str>>();

        moves.push(Move::new(move_input_split[0], move_input_split[1]));

        return moves;
    })
}

fn calculate_move_tail_direction(head_coord: Coord, tail_coord: Coord) -> Option<String> {
    if (head_coord.x - tail_coord.x).abs() == 1 && head_coord.y == tail_coord.y {
        // adjacent horizontally
        return None;
    } else if (head_coord.y - tail_coord.y).abs() == 1 && head_coord.x == tail_coord.x {
        // adjacent vertically
        return None;
    } else if (head_coord.x - tail_coord.x).abs() == 1 && (head_coord.y - tail_coord.y).abs() == 1 {
        // diagonally adjacent
        return None;
    } else if (head_coord.x - tail_coord.x) > 0 && (head_coord.y - tail_coord.y) == 0 {
        return Some(RIGHT.to_string());
    } else if (head_coord.x - tail_coord.x) < 0 && (head_coord.y - tail_coord.y) == 0 {
        return Some(LEFT.to_string());
    } else if (head_coord.x - tail_coord.x) == 0 && (head_coord.y - tail_coord.y) > 0 {
        return Some(UP.to_string());
    } else if (head_coord.x - tail_coord.x) == 0 && (head_coord.y - tail_coord.y) < 0 {
        return Some(DOWN.to_string());
    } else if (head_coord.x - tail_coord.x) == 1 && (head_coord.y - tail_coord.y) == 2 {
        return Some(UP_RIGHT.to_string());
    } else if (head_coord.x - tail_coord.x) == -1 && (head_coord.y - tail_coord.y) == -2 {
        return Some(DOWN_LEFT.to_string());
    } else if (head_coord.x - tail_coord.x) == -1 && (head_coord.y - tail_coord.y) == 2 {
        return Some(UP_LEFT.to_string());
    } else if (head_coord.x - tail_coord.x) == 1 && (head_coord.y - tail_coord.y) == -2 {
        return Some(DOWN_RIGHT.to_string());
    } else if (head_coord.x - tail_coord.x) == 2 && (head_coord.y - tail_coord.y) == 1 {
        return Some(UP_RIGHT.to_string());
    } else if (head_coord.x - tail_coord.x) == -2 && (head_coord.y - tail_coord.y) == -1 {
        return Some(DOWN_LEFT.to_string());
    } else if (head_coord.x - tail_coord.x) == -2 && (head_coord.y - tail_coord.y) == 1 {
        return Some(UP_LEFT.to_string());
    } else if (head_coord.x - tail_coord.x) == 2 && (head_coord.y - tail_coord.y) == -1 {
        return Some(DOWN_RIGHT.to_string());
    }

    return None;
}

fn calculate_tail_coords(moves: Vec<Move>) -> Vec<Coord> {
    let initial_coord = Coord::new(0, 0);
    let mut head_coord = initial_coord.clone();
    let mut tail_coord = initial_coord;

    moves
        .iter()
        .fold(vec![Coord::new(0, 0)], |mut tail_coords, current_move| {
            for _ in 0..current_move.amount {
                if current_move.direction == RIGHT.to_string() {
                    head_coord.move_right();
                } else if current_move.direction == LEFT.to_string() {
                    head_coord.move_left();
                } else if current_move.direction == UP.to_string() {
                    head_coord.move_up();
                } else if current_move.direction == DOWN.to_string() {
                    head_coord.move_down();
                }

                let tail_direction =
                    calculate_move_tail_direction(head_coord.clone(), tail_coord.clone());

                if tail_direction.is_none() {
                    continue;
                }

                if tail_direction.clone().unwrap() == RIGHT.to_string() {
                    tail_coord.move_right();
                } else if tail_direction.clone().unwrap() == LEFT.to_string() {
                    tail_coord.move_left();
                } else if tail_direction.clone().unwrap() == UP.to_string() {
                    tail_coord.move_up();
                } else if tail_direction.clone().unwrap() == DOWN.to_string() {
                    tail_coord.move_down();
                } else if tail_direction.clone().unwrap() == UP_RIGHT.to_string() {
                    tail_coord.move_up_right();
                } else if tail_direction.clone().unwrap() == UP_LEFT.to_string() {
                    tail_coord.move_up_left();
                } else if tail_direction.clone().unwrap() == DOWN_RIGHT.to_string() {
                    tail_coord.move_down_right();
                } else if tail_direction.clone().unwrap() == DOWN_LEFT.to_string() {
                    tail_coord.move_down_left();
                }

                tail_coords.push(tail_coord.clone());
            }

            return tail_coords;
        })
}

fn unique_coords(coords: Vec<Coord>) -> Vec<Coord> {
    let mut unique_map: HashMap<String, Coord> = HashMap::from([]);
    for coord in coords {
        unique_map.insert(format!("[{}, {}]", coord.x, coord.y), coord);
    }

    unique_map.iter().fold(vec![], |mut coords, (_, coord)| {
        coords.push(coord.clone());
        coords
    })
}

pub fn day9_output() {
    let problem_input = read_input("./src/input/files/day9.txt");
    let moves = parse_input(problem_input);
    let mut coords = calculate_tail_coords(moves);

    coords = unique_coords(coords);

    println!("day 9, problem 1: {:?}", coords.len());
    println!("day 9, problem 2: {}", "N/A");
}
