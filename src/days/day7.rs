use crate::input::lib::read_input;
use std::{cmp, collections::HashMap};

#[derive(Debug, Clone)]
struct File {
    size: usize,
}

#[derive(Debug, Clone)]
struct Directory {
    files: Vec<File>,
    nested_directories: Vec<String>,
}

impl File {
    pub fn new(size: usize) -> Self {
        File { size }
    }
}

impl Directory {
    pub fn new() -> Self {
        Directory {
            files: vec![],
            nested_directories: vec![],
        }
    }

    pub fn add_files(&mut self, files: Vec<File>) {
        self.files = files;
    }

    pub fn add_nested_directories(
        &mut self,
        current_path: String,
        nested_directories: Vec<String>,
    ) {
        self.nested_directories = nested_directories
            .iter()
            .map(|nested_dir| format!("{}/{}", current_path, nested_dir))
            .collect::<Vec<String>>();
    }

    pub fn files_sum(&self) -> usize {
        self.files.clone().iter().fold(0, |mut sum, file| {
            sum += file.size;
            return sum;
        })
    }
}

fn collect_ls_output(
    terminal_output: Vec<&str>,
    mut current_line_index: usize,
) -> (Vec<File>, Vec<String>, usize) {
    let mut files: Vec<File> = vec![];
    let mut directories: Vec<String> = vec![];

    while let Some(next_line) = terminal_output.get(current_line_index) {
        if next_line.starts_with("$") {
            break; // handle in if statement outside this loop and condition block
        } else if next_line.starts_with("dir ") {
            let dirname = next_line.replace("dir ", "");
            directories.push(dirname);
        } else {
            // file
            let file_split = next_line.split(" ").collect::<Vec<&str>>();

            files.push(File::new(file_split[0].parse::<usize>().unwrap()));
        }

        current_line_index += 1;
    }

    return (files, directories, current_line_index);
}

fn parse_input(terminal_output: Vec<&str>) -> HashMap<String, Directory> {
    let mut current_output_index = 0;
    let mut current_path: Vec<String> = vec![];

    // hash map assumes all directories are uniquely named across the whole fs (obviously not the case)
    let mut directories: HashMap<String, Directory> = HashMap::from([]);

    while let Some(current_output_line) = terminal_output.get(current_output_index) {
        if current_output_line.starts_with("$ cd ") {
            let dirname = current_output_line.replace("$ cd ", "");

            if dirname != ".." {
                current_path.push(dirname.clone());
                directories.insert(current_path.join("/"), Directory::new());
            } else {
                current_path.pop();
            }
            current_output_index += 1;
        } else if current_output_line.starts_with("$ ls") {
            let current_dir = directories.get_mut(&current_path.join("/")).unwrap();

            let (files, directories, new_output_index) =
                collect_ls_output(terminal_output.clone(), current_output_index + 1);

            current_output_index = new_output_index;
            current_dir.add_files(files);
            current_dir.add_nested_directories(current_path.join("/"), directories);
        }
    }

    return directories;
}

fn nested_directory_sum(directories: HashMap<String, Directory>, directory: Directory) -> usize {
    directory
        .nested_directories
        .iter()
        .fold(0, |mut sum, nested_dirname| {
            if let Some(nested_dir) = directories.get(nested_dirname) {
                sum += directories.get(nested_dirname).unwrap().files_sum();

                if !nested_dir.nested_directories.is_empty() {
                    sum += nested_directory_sum(directories.clone(), nested_dir.clone());
                }
            }

            return sum;
        })
}

fn directory_sums(directories: HashMap<String, Directory>) -> HashMap<String, usize> {
    directories
        .iter()
        .fold(HashMap::from([]), |mut sums, (dirname, directory)| {
            let mut sum = 0;
            sum += directory.files_sum();
            sum += nested_directory_sum(directories.clone(), directory.clone());

            sums.insert(dirname.clone(), sum);
            return sums;
        })
}

pub fn sum_of_directories_with_max_size(
    directory_sums: HashMap<String, usize>,
    max_size: usize,
) -> usize {
    directory_sums.iter().fold(0, |mut sum, (_, dir_sum)| {
        if dir_sum.clone() <= max_size {
            sum += dir_sum;
        }
        return sum;
    })
}

pub fn smallest_directory_size_with_min_size(
    directory_sums: HashMap<String, usize>,
    min_size: usize,
) -> usize {
    directory_sums.iter().fold(0, |mut size, (_, dir_sum)| {
        if dir_sum.clone() >= min_size {
            if size == 0 {
                size = dir_sum.clone();
            } else {
                size = cmp::min(dir_sum.clone(), size);
            }
        }

        return size;
    })
}

pub fn day7_output() {
    let problem_input = read_input("./src/input/files/day7.txt");
    let terminal_output = problem_input.split("\n").collect::<Vec<&str>>();
    let directories = parse_input(terminal_output);

    let directory_sums = directory_sums(directories.clone());

    // Part 1
    let sum = sum_of_directories_with_max_size(directory_sums.clone(), 100000);

    // Total fs size = 70000000
    // Total root size = 48008081
    // Required space for update = 30000000
    // Current unused space = 21991919
    // min space required to free = 8008081
    // Part 2
    let smallest_directory_size =
        smallest_directory_size_with_min_size(directory_sums.clone(), 8008081);

    println!("day 7, problem 1: {}", sum);
    println!("day 7, problem 2: {}", smallest_directory_size);
}
