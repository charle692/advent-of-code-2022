mod days {
    pub mod day1;
    pub mod day2;
}

mod input {
    pub mod lib;
}

fn main() {
    days::day1::total_calories_for_top_3();
    days::day2::total_score_for_strategy_guide();
}
