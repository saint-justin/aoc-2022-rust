pub mod solutions {
  use std::{collections::{HashSet, HashMap}, ops::AddAssign};

  #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
  struct Position(i16, i16); // Simple struct for x/y

  impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Position (self.0 + other.0, self.1 + other.1);
    }
}

  /// Day 9 Part 1 -- https://adventofcode.com/2022/day/9
  /// 
  /// Your input is a series of movements of a given rope, you need
  /// to use the movements of the head of the rope to track where the 
  /// tail follows. Calculate the total amount of positions the tail
  /// enters over the course of the input.
  pub fn find_tail_positions(head_movements: &Vec<&str>) {
    let mut position_set: HashSet<Position> = HashSet::new();
    let mut head_pos = Position(0, 0);
    let mut tail_pos = Position(0, 0);

    position_set.insert(tail_pos);

    let movement_dict: HashMap<String, Position> = HashMap::from([
      ("u".to_owned(), Position(0, 1)),
      ("d".to_owned(), Position(0, -1)),
      ("r".to_owned(), Position(1, 0)),
      ("l".to_owned(), Position(-1, 0))
    ]);

    for command in head_movements {
      let args: Vec<&str> = command.split(" ").collect();
      let direction = movement_dict.get(args[0]).unwrap();

      for _ in 0..String::from(args[1]).parse::<i16>().unwrap() {
        head_pos += *direction;
        let x_diff = (head_pos.0 - tail_pos.0).abs();
        let y_diff = (head_pos.1 - tail_pos.1).abs();
        if x_diff > 1 {
          
        }
      }
    }
  }
}