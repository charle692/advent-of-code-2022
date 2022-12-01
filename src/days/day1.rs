use crate::input::lib::read_input;

fn calories_per_elf(input_vec: Vec<&str>) -> Vec<i64> {
    let mut calories_per_elf: Vec<i64> = vec![];
    let mut current_elf_calorie_count: i64 = 0;

    for &input in input_vec.iter() {
        if input == "" {
            calories_per_elf.push(current_elf_calorie_count);
            current_elf_calorie_count = 0;
        } else {
            current_elf_calorie_count += input.parse::<i64>().unwrap();
        }
    }

    return calories_per_elf;
}

pub fn total_calories_for_top_3() {
    let problem_input = read_input("./src/input/files/day1_problem1.txt");
    let input_vec = problem_input.split("\n").collect::<Vec<&str>>();

    let mut elf_calorie_count = calories_per_elf(input_vec);
    elf_calorie_count.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let top_3_elves_total = elf_calorie_count.get(0).unwrap()
        + elf_calorie_count.get(1).unwrap()
        + elf_calorie_count.get(2).unwrap();

    println!("{}", top_3_elves_total);
}
