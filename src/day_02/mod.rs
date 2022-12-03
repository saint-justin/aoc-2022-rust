pub mod solution {
    use std::collections::HashMap;

  /// Day 2 Part 1
  /// https://adventofcode.com/2022/day/2
  /// 
  /// Part 1 input is 2 parts: (A/B/C) (X/Y/Z)
  /// OUTCOME:      THROWS:
  /// Lose     +0   Rock     +1
  /// Tie      +3   Paper    +2
  /// Win      +6   Scissors +3
  ///
  pub fn calculate_total_score(encrypted_strategy_guide: &Vec<&str>) {
    let mut points = 0;
    let match_dict = build_match_dictionary();
    encrypted_strategy_guide
      .iter()
      .for_each(|encrypted_match| {
        if match_dict.contains_key(*encrypted_match) {
          points += match_dict.get(*encrypted_match).unwrap();
        }

        let choices: Vec<&str> = encrypted_match.split(" ").collect();
        points += get_points_for_choice(choices[1]);
      }
    );

    println!("Points gathered: {points}");
  }

  /// Day 2 Part 1
  /// https://adventofcode.com/2022/day/2
  /// 
  /// Part 1 input is 2 parts: (A/B/C) (X/Y/Z)
  /// A/B/C is what the opponent will throw (rock/paper/scissors)
  /// X/Y/Z is what the outcome needs to be (lose/tie/win)
  ///
  pub fn calculate_total_score_adjusted(encrypted_strategy_guide: &Vec<&str>) {
    let mut points = 0;
    let match_dict = build_decision_dict();
    encrypted_strategy_guide
      .iter()
      .for_each(|encrypted_match| {
        if match_dict.contains_key(*encrypted_match) {
          points += match_dict.get(*encrypted_match).unwrap();
        }
      }
    );

    println!("Points gathered: {points}");
  }

  /// 0 -> lose
  /// 1 -> tie
  /// 2 -> win
  fn build_match_dictionary() -> HashMap<String, i32> {
    return HashMap::from([
      (String::from("A X"), 3), // R/R
      (String::from("A Y"), 6), // R/P
      (String::from("A Z"), 0), // R/S
      (String::from("B X"), 0), // P/R
      (String::from("B Y"), 3), // P/P
      (String::from("B Z"), 6), // P/S
      (String::from("C X"), 6), // R/R
      (String::from("C Y"), 0), // R/P
      (String::from("C Z"), 3), // R/S
    ]);
  }

  // Format: <InputKey, L/T/W + Points for Choice>
  fn build_decision_dict() -> HashMap<String, i32> {
    return HashMap::from([
      (String::from("A X"), 0 + 3), // Opponent throws R, to Lose throw -> S
      (String::from("A Y"), 3 + 1), // Opponent throws R, to Tie  throw -> R
      (String::from("A Z"), 6 + 2), // Opponent throws R, to Win  throw -> P
      (String::from("B X"), 0 + 1), // Opponent throws P, to Lose throw -> R
      (String::from("B Y"), 3 + 2), // Opponent throws P, to Tie  throw -> P
      (String::from("B Z"), 6 + 3), // Opponent throws P, to Win  throw -> S
      (String::from("C X"), 0 + 2), // Opponent throws S, to Lose throw -> P
      (String::from("C Y"), 3 + 3), // Opponent throws S, to Tie  throw -> S
      (String::from("C Z"), 6 + 1), // Opponent throws S, to Win  throw -> R
    ]);
  }

  fn get_points_for_choice(choice: &str) -> i32 {
    match choice {
      "X" => return 1,
      "Y" => return 2,
      "Z" => return 3,
      _ => panic!("Invalid choice!")
    }
  }

}