use crate::input::lib::read_input;

const ITEMS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_priority(item: char, priorities: Vec<char>) -> usize {
    return priorities.iter().position(|&p| p == item).unwrap() + 1;
}

fn priority_sum(rucksacks: Vec<&str>, priorities: Vec<char>) -> usize {
    let priority_sum = rucksacks.iter().fold(0, |mut priority_sum, rucksack| {
        let rucksack_vec = rucksack.chars().collect::<Vec<char>>();
        let (comp1, comp2) = rucksack_vec.split_at(rucksack_vec.len() / 2);

        for item in comp1 {
            if comp2.contains(item) {
                priority_sum += get_priority(item.clone(), priorities.clone());
                break;
            }
        }

        return priority_sum;
    });

    return priority_sum;
}

fn badge_priority_sum(rucksacks: Vec<&str>, priorities: Vec<char>) -> usize {
    let mut current_group = 0;
    let mut priority_sum = 0;

    while current_group < rucksacks.len() - 1 {
        let rucksack1 = rucksacks[current_group].chars().collect::<Vec<char>>();
        let rucksack2 = rucksacks[current_group + 1].chars().collect::<Vec<char>>();
        let rucksack3 = rucksacks[current_group + 2].chars().collect::<Vec<char>>();

        for item in rucksack1 {
            if rucksack2.contains(&item) && rucksack3.contains(&item) {
                priority_sum += get_priority(item.clone(), priorities.clone());
                current_group += 3;
                break;
            }
        }
    }

    return priority_sum;
}

pub fn sum_of_priorities_for_duplicates() {
    let problem_input = read_input("./src/input/files/day3.txt");
    let rucksacks = problem_input.split("\n").collect::<Vec<&str>>();
    let priorities = ITEMS.chars().collect::<Vec<char>>();

    let part1 = priority_sum(rucksacks.clone(), priorities.clone());
    let part2 = badge_priority_sum(rucksacks.clone(), priorities.clone());

    println!("day 3, problem 1: {}", part1);
    println!("day 3, problem 2: {}", part2);
}
