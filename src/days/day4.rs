use crate::input::lib::read_input;
use std::cmp;

fn section_assignments(assignment_pair: &str) -> (i32, i32, i32, i32) {
    let assignments = assignment_pair.split(&['-', ',']).collect::<Vec<&str>>();

    return (
        assignments[0].parse::<i32>().unwrap(),
        assignments[1].parse::<i32>().unwrap(),
        assignments[2].parse::<i32>().unwrap(),
        assignments[3].parse::<i32>().unwrap(),
    );
}

fn fully_overlapping_pairs(assignment_pairs: Vec<&str>) -> usize {
    return assignment_pairs
        .iter()
        .fold(0, |mut full_overlap_count, assignment_pair| {
            let (e1_s1, e1_s2, e2_s1, e2_s2) = section_assignments(assignment_pair);

            if (e1_s1 <= e2_s1 && e1_s2 >= e2_s2) || (e2_s1 <= e1_s1 && e2_s2 >= e1_s2) {
                full_overlap_count += 1;
            }

            return full_overlap_count;
        });
}

fn partial_overllaping_pairs(assignment_pairs: Vec<&str>) -> usize {
    return assignment_pairs
        .iter()
        .fold(0, |mut part_overlap_count, assignment_pair| {
            let (e1_s1, e1_s2, e2_s1, e2_s2) = section_assignments(assignment_pair);

            if cmp::max(e1_s1, e2_s1) <= cmp::min(e1_s2, e2_s2) {
                part_overlap_count += 1;
            }

            return part_overlap_count;
        });
}

pub fn day4_output() {
    let problem_input = read_input("./src/input/files/day4.txt");
    let assignment_pairs = problem_input.split("\n").collect::<Vec<&str>>();

    let full_overlap_count = fully_overlapping_pairs(assignment_pairs.clone());
    let part_overlap_count = partial_overllaping_pairs(assignment_pairs.clone());

    println!("Day 4, problem 1: {}", full_overlap_count);
    println!("Dat 4, problem 2: {}", part_overlap_count);
}
