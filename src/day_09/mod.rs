#[allow(unused)]
pub mod solutions {
  use std::{collections::{HashSet, HashMap}, ops::{AddAssign, Sub}, env::args};

  #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
  struct Position(i16, i16); // Simple struct for x/y

  impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Position (self.0 + other.0, self.1 + other.1);
    }
  }

  impl Sub for Position {
    fn sub(self, other: Self) -> Position {
        return Position (self.0 - other.0, self.1 - other.1);
    }

    type Output = Position;
  }

  /// Day 9 Part 1 -- https://adventofcode.com/2022/day/9
  /// 
  /// Your input is a series of movements of a given rope, you need
  /// to use the movements of the head of the rope to track where the 
  /// tail follows. Calculate the total amount of positions the tail
  /// enters over the course of the input.
  pub fn find_tail_positions(head_movements: &Vec<&str>) {
    let mut head_pos = Position(0, 0);
    let mut tail_pos = Position(0, 0);
    let mut position_set: HashSet<Position> = HashSet::from([tail_pos]);
    let movement_dict: HashMap<String, Position> = generate_movement_dict();

    for command in head_movements {
      let args: Vec<&str> = command.split(" ").collect();
      let direction = movement_dict.get(args[0]).unwrap();

      for _ in 0..String::from(args[1]).parse::<i16>().unwrap() {
        head_pos += *direction;
        tail_pos = calculate_next_postion(&head_pos, &tail_pos, direction);
        position_set.insert(tail_pos);
      }
    }

    println!("Total Positions Tail Visited: {}", position_set.len());
  }

  /// Day 9 Part 2 -- https://adventofcode.com/2022/day/9
  /// 
  /// Same input as part 1, except this time around you need to track
  /// 10 segments of rope (including head + tail) instead of 
  pub fn find_tail_positions_with_extra_lengths(head_movements: &Vec<&str>) {
    let mut position_set: HashSet<Position> = HashSet::from([Position(0, 0)]);
    let mut all_knots: Vec<Position> = Vec::new();
    for _ in 0..10 { all_knots.push(Position(0, 0)); }

    println!("Knot count: {}", all_knots.len());
  
    let movement_dict: HashMap<String, Position> = generate_movement_dict();

    for command in head_movements {
      let args: Vec<&str> = command.split(" ").collect();
      let direction = movement_dict.get(args[0]).unwrap();

      for _ in 0..String::from(args[1]).parse::<i16>().unwrap() {
        let mut pos_diff = direction;
        all_knots[0] += *pos_diff;
        // Iterate over each knot
        for i in 0..9 {
          let head = all_knots[i];
          let tail = all_knots[i + 1];
          let next_tail_pos = calculate_next_postion(&head, &tail, pos_diff);
          pos_diff = &(next_tail_pos - tail);
          
          all_knots[i + 1] = next_tail_pos;
        }
        // head_pos += *direction;
        // let next_pos = calculate_next_postion(&head_pos, &tail_pos, direction);
        // position_set.insert(tail_pos);
      }
    }

    println!("Total Positions Tail Visited: {}", position_set.len());
  }

  fn calculate_next_postion(lead: &Position, follow_original: &Position, direction: &Position) -> Position {
    let mut follow = follow_original.clone();
    let pos_diff = Position(lead.0 - follow.0, lead.1 - follow.1) ;

    // If we start at a diag and move that same diag, then adjust as needed
    if pos_diff.0.abs() + pos_diff.1.abs() == 2 {
      if pos_diff == *direction {
        follow += pos_diff;
        return follow;
      }
    }

    // Otherwise, just adjust movement as usual
    if pos_diff.0 >  1 { follow += Position( 1, 0) }
    if pos_diff.0 < -1 { follow += Position(-1, 0) }
    if pos_diff.0.abs() > 1 {
      if direction.0.abs() > 0 {follow = Position(follow.0, lead.1)}
    }

    if pos_diff.1 >  1 { follow += Position(0,  1) }
    if pos_diff.1 < -1 { follow += Position(0, -1) }
    if pos_diff.1.abs() > 1 {
      if direction.1.abs() > 0 {follow = Position(lead.0, follow.1)}
    }

    return follow;
  }

  fn generate_movement_dict() -> HashMap<String, Position> {
    return HashMap::from([
      ("U".to_owned(), Position(0, 1)),
      ("D".to_owned(), Position(0, -1)),
      ("R".to_owned(), Position(1, 0)),
      ("L".to_owned(), Position(-1, 0))
    ]);
  }
}