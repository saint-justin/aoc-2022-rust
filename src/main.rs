use std::{fs};

mod day_01;

fn main() {
    let contents = fs::read_to_string("./src/day_01/input.txt")
      .expect("Should have been able to read the file");

    let puzzle_input: Vec<&str> = contents
      .split(['\n'])
      .map(|e| e.trim())
      .collect();

    // day_01::solutions::find_elf_carrying_most_calories(&puzzle_input);
    day_01::solutions::find_top_three_calorie_sum(&puzzle_input);
}