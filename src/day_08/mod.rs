pub mod solutions {
  #[derive(Debug, Clone, Copy)]
  struct Score(usize, usize, usize, usize);

  #[derive(Debug, Clone, Copy)]
  struct Position(usize, usize);

  /// Day 8 Part 1 -- https://adventofcode.com/2022/day/8
  /// 
  /// Your input is a heightmap of trees in a forest, each number
  /// represents the height of a single tree in meters (0 is short tree,
  /// 9 is very tall tree). How many trees are visible from outside the 
  /// forest in any of the 4 directions?
  pub fn get_visible_tree_count(input_heightmap: &Vec<&str>) {
    let typed_heightmap: Vec<Vec<i16>> = input_heightmap.iter()
      .map(|row| row.chars().into_iter()
        .map(|ch| String::from(ch).parse().unwrap())
        .collect::<Vec<i16>>())
      .collect();

    let mut total = 0;
    for row in 0..typed_heightmap.len() {
      for col in 0..typed_heightmap[0].len() {
        if tree_is_visible(Position(row, col), typed_heightmap[row][col], &typed_heightmap) {
          // println!("Tree is visible!");
          total += 1;
        } else {
          // println!("Tree is not visible!");
        }
      }
    }

    println!("Total: {total}");
  }

  /// Day 8 Part 2 -- https://adventofcode.com/2022/day/8#part2
  /// 
  /// Same input as day 1, except instead of finding the number of visible
  /// trees total, you need to find the _best_ tree. The best tree is the one with
  pub fn get_highest_scenic_score_tree(input_heightmap: &Vec<&str>) {
    let typed_heightmap: Vec<Vec<i16>> = input_heightmap.iter()
      .map(|row| row.chars().into_iter()
        .map(|ch| String::from(ch).parse().unwrap())
        .collect::<Vec<i16>>())
      .collect();

    let mut highest_score_sum = 0;
    let mut highest_score_set = Score(0, 0, 0, 0);
    for row in 0..typed_heightmap.len() {
      for col in 0..typed_heightmap[0].len() {
        let score = calculate_tree_score(Position(row, col), typed_heightmap[row][col], &typed_heightmap);
        let score_sum = score.0 * score.1 * score.2 * score.3;
        if score_sum > highest_score_sum {
          println!("New score sum max: {score_sum}");
          highest_score_sum = score_sum;
          highest_score_set = score;
        }
      }
    }

    println!("Highest scoring tree: Sum {highest_score_sum}  Components: {highest_score_set:?}")
  }

  fn tree_is_visible(position: Position, tree_value: i16, typed_heightmap: &Vec<Vec<i16>>) -> bool {
    let mut current: Position = position;
    let mut left = true;
    while current.0 > 0 {
      current = Position(current.0 - 1, current.1);
      if typed_heightmap[current.0][current.1] >= tree_value { left = false; }
    }

    current = position;
    let mut right = true;
    while current.0 < typed_heightmap.len() - 1 {
      current = Position(current.0 + 1, current.1);
      if typed_heightmap[current.0][current.1] >= tree_value { right = false; }
    }

    current = position;
    let mut up = true;
    while current.1 > 0 {
      current = Position(current.0, current.1 - 1);
      if typed_heightmap[current.0][current.1] >= tree_value { up = false; }
    }

    current = position;
    let mut down = true;
    while current.1 < typed_heightmap[0].len() - 1 {
      current = Position(current.0, current.1 + 1);
      if typed_heightmap[current.0][current.1] >= tree_value { down = false; }
    }

    return left || right || up || down;
  }

  fn calculate_tree_score(position: Position, tree_value: i16, typed_heightmap: &Vec<Vec<i16>>) -> Score {
    let mut current: Position = position;
    let mut left = 0;
    while current.0 > 0 {
      current = Position(current.0 - 1, current.1);
      left += 1;
      if typed_heightmap[current.0][current.1] >= tree_value { break }
    }

    current = position;
    let mut right = 0;
    while current.0 < typed_heightmap.len() - 1 {
      current = Position(current.0 + 1, current.1);
      right += 1;
      if typed_heightmap[current.0][current.1] >= tree_value { break }
    }

    current = position;
    let mut up = 0;
    while current.1 > 0 {
      current = Position(current.0, current.1 - 1);
      up += 1;
      if typed_heightmap[current.0][current.1] >= tree_value { break }
    }

    current = position;
    let mut down = 0;
    while current.1 < typed_heightmap[0].len() - 1 {
      current = Position(current.0, current.1 + 1);
      down += 1;
      if typed_heightmap[current.0][current.1] >= tree_value { break }
    }

    return Score(left, right, up, down);
  }
}