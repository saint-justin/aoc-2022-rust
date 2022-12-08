use std::{fs};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

fn main() {
    let contents = fs::read_to_string("./src/day_06/input.txt")
      .expect("Should have been able to read the file");

    let puzzle_input: Vec<&str> = contents
      .split(['\n'])
      .map(|e| e.trim())
      .collect();

    // day_01::solutions::find_elf_carrying_most_calories(&puzzle_input);
    // day_01::solutions::find_top_three_calorie_sum(&puzzle_input);
    // day_02::solution::calculate_total_score(&puzzle_input);
    // day_02::solution::calculate_total_score_adjusted(&puzzle_input);
    // day_03::solutions::find_priority_sum_of_dups(&puzzle_input);
    // day_03::solutions::find_priority_sum_of_team_badges(&puzzle_input);
    // day_04::solutions::find_fully_contained_assignments(&puzzle_input);
    // day_04::solutions::find_partially_contained_assignments(&puzzle_input);
    // day_05::solutions::find_rearrangement_message(&puzzle_input);
    // day_05::solutions::find_multimove_message(&puzzle_input);
    // day_06::solutions::characters_before_start_of_packet(&puzzle_input);
    day_06::solutions::characters_before_start_of_packet_big(&puzzle_input);
}