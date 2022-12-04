#[allow(unused)]
pub mod solutions {

  /// Day 1 Part 1 -- https://adventofcode.com/2022/day/1
  /// 
  /// The input is a series of calorie counts, breaks in entries indicate
  /// a new elf. Calculate which elf is carrying the most calories and
  /// return that elf's carried calorie count.
  pub fn find_elf_carrying_most_calories(elf_food_logs: &Vec<&str>) {
    let mut i: usize = 0;
    let mut current_elf_calories: u32 = 0;
    let mut most_calories_so_far: u32 = 0;

    while i < elf_food_logs.len() {
      // Set up for new elf if the line is empty
      if elf_food_logs[i].len() == 0 {
        if current_elf_calories > most_calories_so_far {
          most_calories_so_far = current_elf_calories;
        }
        current_elf_calories = 0;
      } else {
        let item_calories: u32 = elf_food_logs[i].parse::<u32>().unwrap();
        current_elf_calories += item_calories;
      }
      i += 1;
    }

    println!("Highest Calorie Count: {most_calories_so_far}");
  }

  /// Day 1 Part 2 -- https://adventofcode.com/2022/day/1#part2
  /// Same as above, but for the top 3 elves instead of just the firsts
  pub fn find_top_three_calorie_sum(elf_food_logs: &Vec<&str>) {
    let mut i: usize = 0;
    let mut current_elf_calories: u32 = 0;
    let mut most_calories_so_far: Vec<u32> = vec![0, 0, 0]; // (highest -> lowest)

    while i < elf_food_logs.len() {
      // Set up for new elf if the line is empty
      if elf_food_logs[i].len() == 0 {
        if current_elf_calories > most_calories_so_far[2] {
          most_calories_so_far.push(current_elf_calories);
          most_calories_so_far.sort_by(|a,b| b.cmp(a));
          if most_calories_so_far.len() > 3 {
            most_calories_so_far.pop();
          }
        }
        current_elf_calories = 0;
      } 
      // Otherwise just thiccen up the current elf
      else {
        let item_calories: u32 = elf_food_logs[i].parse::<u32>().unwrap();
        current_elf_calories += item_calories;
      }

      i += 1;
    }

    println!("Highest calorie elves: {most_calories_so_far:?}");
    println!("Calorie Count Sum: {}", most_calories_so_far.iter().sum::<u32>());
  }
}