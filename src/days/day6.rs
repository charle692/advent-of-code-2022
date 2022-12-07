use std::collections::HashMap;

use crate::input::lib::read_input;

fn start_of_x_marker(data_stream_chars: Vec<char>, num_distinct_chars: usize) -> Option<usize> {
    let max_window_start_position = data_stream_chars.len() - (num_distinct_chars - 1);

    for window_start_position in 0..max_window_start_position {
        let mut window_characters: HashMap<char, char> = HashMap::new();
        let window_size = window_start_position + num_distinct_chars;

        for char_number in window_start_position..window_size {
            let current_character = data_stream_chars[char_number];

            if window_characters.contains_key(&current_character) {
                break;
            } else {
                window_characters.insert(current_character, current_character);
            }
        }

        if window_characters.len() == num_distinct_chars {
            return Some(window_size);
        }

        window_characters.clear();
    }

    return None;
}

pub fn day6_output() {
    let problem_input = read_input("./src/input/files/day6.txt");
    let data_stream_chars = problem_input.chars().collect::<Vec<char>>();

    let start_packet_marker = start_of_x_marker(data_stream_chars.clone(), 4);
    let start_message_marker = start_of_x_marker(data_stream_chars.clone(), 14);

    println!("day 6, problem 1: {}", start_packet_marker.unwrap_or(0));
    println!("day 6, problem 2: {}", start_message_marker.unwrap_or(0));
}
