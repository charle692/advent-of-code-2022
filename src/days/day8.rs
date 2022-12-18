use crate::input::lib::read_input;

fn create_tree_matrix(problem_input: String) -> Vec<Vec<u32>> {
    let tree_rows_input = problem_input.split("\n").collect::<Vec<&str>>();

    tree_rows_input
        .iter()
        .fold(vec![], |mut rows, tree_row_input| {
            let trees = tree_row_input
                .chars()
                .map(|tree_size| tree_size.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            rows.push(trees);

            return rows;
        })
}

fn visible_trees(tree_size_matrix: Vec<Vec<u32>>) -> i32 {
    let num_rows: i32 = (tree_size_matrix.len() - 1) as i32;
    let num_cols: i32 = (tree_size_matrix[0].len() - 1) as i32;
    let mut visible_trees: i32 = num_rows * 2 + num_cols * 2;

    for (row_num, tree_size_row) in tree_size_matrix.iter().enumerate() {
        if row_num == 0 || row_num == tree_size_matrix.len() - 1 {
            continue;
        }

        for (tree_num, tree) in tree_size_row.iter().enumerate() {
            let mut hidden_right = false;
            let mut hidden_left = false;
            let mut hidden_below = false;
            let mut hidden_above = false;

            if tree_num == 0 || tree_num == tree_size_row.len() - 1 {
                continue;
            }

            // check if any other tree right is smaller
            for i in (tree_num + 1)..tree_size_row.len() {
                if tree <= &tree_size_row[i] {
                    hidden_right = true;
                    break;
                }
            }

            // check if any other tree left is smaller
            for i in 0..tree_num {
                if tree <= &tree_size_row[i] {
                    hidden_left = true;
                    break;
                }
            }

            // check if any other tree below is smaller
            for i in (row_num + 1)..tree_size_matrix.len() {
                if tree <= &tree_size_matrix[i][tree_num] {
                    hidden_below = true;
                    break;
                }
            }

            // check if any other tree allow is smaller
            for i in 0..row_num {
                if tree <= &tree_size_matrix[i][tree_num] {
                    hidden_above = true;
                    break;
                }
            }

            if !(hidden_right && hidden_left && hidden_below && hidden_above) {
                visible_trees += 1;
            }
        }
    }

    return visible_trees;
}

pub fn day8_output() {
    let problem_input = read_input("./src/input/files/day8.txt");
    let tree_size_matrix = create_tree_matrix(problem_input);
    let visible_tree_count = visible_trees(tree_size_matrix);

    println!("day 8, problem 1: {}", visible_tree_count);
    println!("day 8, problem 2: {}", "N/A");
}
