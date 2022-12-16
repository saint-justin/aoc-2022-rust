use std::{fs};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_12;

fn main() {
    let contents = fs::read_to_string("./src/day_12/input_test.txt")
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
    // day_06::solutions::characters_before_start_of_packet_big(&puzzle_input);
    // day_08::solutions::get_visible_tree_count(&puzzle_input);
    // day_08::solutions::get_highest_scenic_score_tree(&puzzle_input);
    // day_09::solutions::find_tail_positions(&puzzle_input);
    // day_10::solutions::sum_six_signal_strengths(&puzzle_input);
    // day_10::solutions::print_crt_display(&puzzle_input);
    day_12::solutions::find_shortest_path(&puzzle_input);

    // TODO: 
    // Day 7 All
    // Day 9 Part 2
    // Day 11 ALL
}