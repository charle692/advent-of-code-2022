use crate::input::lib::read_input;
use regex::Regex;

#[derive(Clone)]
struct Move {
    quantity: usize,
    origin_stack: usize,
    destination_stack: usize,
}

fn split_problem_input<'a>(problem_input: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {
    let lines = problem_input.split("\n").collect::<Vec<&str>>();
    let (stacks, steps) = lines.split_at(8);

    let mut sanitized_steps = steps.to_vec();

    sanitized_steps.remove(0);
    sanitized_steps.remove(0);

    return (stacks.to_vec(), sanitized_steps);
}

fn convert_stacks_input_into_vectors(stacks_input: Vec<&str>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    for stack in stacks_input.iter().rev() {
        let stack_chars = stack.chars().collect::<Vec<char>>();

        for (i, &stack_crate) in stack_chars.iter().enumerate() {
            if stack_crate != ' ' && stack_crate != '[' && stack_crate != ']' {
                stacks[(i - 1) / 4].push(stack_crate);
            }
        }
    }

    return stacks;
}

fn convert_steps_input_into_moves(steps_input: Vec<&str>) -> Vec<Move> {
    let init_moves: Vec<Move> = vec![];
    let re = Regex::new(r"(move )|( from )|( to )").unwrap();

    return steps_input.iter().fold(init_moves, |mut moves, step| {
        let sanitized_step = re.replace_all(step, ",");
        let step_vector = sanitized_step.split(",").collect::<Vec<&str>>();

        moves.push(Move {
            quantity: step_vector[1].parse::<usize>().unwrap() as usize,
            origin_stack: step_vector[2].parse::<usize>().unwrap() as usize,
            destination_stack: step_vector[3].parse::<usize>().unwrap() as usize,
        });

        return moves;
    });
}

fn move_crates_problem1(stacks: Vec<Vec<char>>, moves: Vec<Move>) -> Vec<Vec<char>> {
    let mut new_stacks = stacks.clone();

    for crate_move in moves {
        for _ in 0..crate_move.quantity {
            let popped_crate = new_stacks[crate_move.origin_stack - 1].pop();

            if popped_crate.is_some() {
                new_stacks[crate_move.destination_stack - 1].push(popped_crate.unwrap());
            }
        }
    }

    return new_stacks;
}

fn split_at(quantity: usize, stack_len: usize) -> usize {
    if quantity >= stack_len {
        return 0;
    }

    return stack_len - quantity;
}

fn move_crates_problem2(stacks: Vec<Vec<char>>, moves: Vec<Move>) -> Vec<Vec<char>> {
    let mut new_stacks = stacks.clone();

    for crate_move in moves {
        let origin_stack_len = new_stacks[crate_move.origin_stack - 1].len();
        if origin_stack_len != 0 {
            let split_off_at = split_at(crate_move.quantity, origin_stack_len);

            let mut crates_to_move =
                new_stacks[crate_move.origin_stack - 1].split_off(split_off_at);
            let crates_to_move_mut = crates_to_move.as_mut();

            new_stacks[crate_move.destination_stack - 1].append(crates_to_move_mut);
        }
    }

    return new_stacks;
}

pub fn day5_output() {
    let problem_input = read_input("./src/input/files/day5.txt");
    let (stacks_input, steps_input) = split_problem_input(problem_input.as_str());
    let stacks = convert_stacks_input_into_vectors(stacks_input);
    let moves = convert_steps_input_into_moves(steps_input);

    let stacks_1 = move_crates_problem1(stacks.clone(), moves.clone());
    let stacks_2 = move_crates_problem2(stacks.clone(), moves.clone());

    println!(
        "day 5, problem 1: {}{}{}{}{}{}{}{}{}",
        stacks_1[0].last().unwrap_or(&' '),
        stacks_1[1].last().unwrap_or(&' '),
        stacks_1[2].last().unwrap_or(&' '),
        stacks_1[3].last().unwrap_or(&' '),
        stacks_1[4].last().unwrap_or(&' '),
        stacks_1[5].last().unwrap_or(&' '),
        stacks_1[6].last().unwrap_or(&' '),
        stacks_1[7].last().unwrap_or(&' '),
        stacks_1[8].last().unwrap_or(&' ')
    );

    println!(
        "day 5, problem 2: {}{}{}{}{}{}{}{}{}",
        stacks_2[0].last().unwrap_or(&' '),
        stacks_2[1].last().unwrap_or(&' '),
        stacks_2[2].last().unwrap_or(&' '),
        stacks_2[3].last().unwrap_or(&' '),
        stacks_2[4].last().unwrap_or(&' '),
        stacks_2[5].last().unwrap_or(&' '),
        stacks_2[6].last().unwrap_or(&' '),
        stacks_2[7].last().unwrap_or(&' '),
        stacks_2[8].last().unwrap_or(&' ')
    );
}
