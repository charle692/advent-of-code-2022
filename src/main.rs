mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
}

mod input {
    pub mod lib;
}

fn main() {
    days::day1::total_calories_for_top_3();
    days::day2::total_score_for_strategy_guide();
    days::day3::sum_of_priorities_for_duplicates();
    days::day4::day4_output();
    days::day5::day5_output();
}
