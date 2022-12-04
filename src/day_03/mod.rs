#[allow(unused)]
pub mod solutions {
  /// Day 3 Part 1 -- https://adventofcode.com/2022/day/3
  /// 
  /// A given rucksack (line of text) has two compartments, the front and back halves.
  /// There's one shared item between the two. Find it, get its "priority" (position in alphabet a-zA-Z)
  /// and return the priority sum of all rucks
  pub fn find_priority_sum_of_dups(all_rucks: &Vec<&str>) {
    let priority_sum: usize = all_rucks.iter()
      .map(|ruck| {
        let ruck_str = String::from(*ruck);
        let compartments = (&ruck_str[..(ruck_str.len() / 2)], &ruck_str[(ruck_str.len() / 2)..]);
        let comparator = String::from(compartments.0);
        let mut oddball: String = String::new();
        compartments.1.chars().for_each(|ch| {
          if comparator.contains(ch) {
            oddball = String::from(ch);
          }
        });
        return get_priority_of(&oddball);
      }).sum();
      println!("Priority Sum: {priority_sum}");
  }

  /// Day 3 Part 2 -- https://adventofcode.com/2022/day/3#part2
  /// 
  /// Same basic pitch as above but ever 3 entries are considered a team,
  /// and you need to find the common character per team and sum their priorities.
  pub fn find_priority_sum_of_team_badges(all_rucks: &Vec<&str>) {
    let mut i = 0;
    let mut badge_priority_sum = 0;
    while i < all_rucks.len() {
      let team_rucks = vec!(all_rucks[i], all_rucks[i+1], all_rucks[i+2]);
      for ch in team_rucks[0].chars().into_iter() {
        if team_rucks[1].contains(ch) && team_rucks[2].contains(ch) {
          badge_priority_sum += get_priority_of(&String::from(ch));
          break;
        }
      };
      i += 3;
    }

    println!("Badge priority sum: {badge_priority_sum}");
  }

  // Helper to return the priority of a given letter
  fn get_priority_of(ch: &String) -> usize {
    let out = String::from("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ").find(ch);
    match out {
      Some(index) => return index,
      None => panic!("Character not found"),
    }
  }

}