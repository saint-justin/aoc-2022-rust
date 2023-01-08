#[allow(unused)]
pub mod solutions {
  #[derive(Debug)]
  struct Position(usize, usize);

  pub fn find_shortest_path(heightmap: &Vec<&str>) {
    let ref_str = String::from("SEabcdefghijklmnopqrstuvwxyz");
    let typed_heightmap: Vec<Vec<usize>> = heightmap.iter()
      .map(|row| {
        return row.chars()
          .map(|ch| ref_str.find(ch).unwrap())
          .collect::<Vec<usize>>();
      }).collect();

    let mut start_pos: Position = Position(usize::MAX, usize::MAX);
    let mut target_pos: Position = Position(usize::MAX, usize::MAX);
    let height = typed_heightmap.len();
    let width = typed_heightmap[0].len();

    for row in 0..height {
      for col in 0..width {
        if typed_heightmap[row][col] == 0 { start_pos = Position(row, col) }
        if typed_heightmap[row][col] == 1 { target_pos = Position(row, col) }
      }
    }

    if vec![start_pos.0, start_pos.1, target_pos.0, target_pos.1].contains(&usize::MAX) {
      panic!("Positions improperly initialized: start {:?}  end {:?}", start_pos, target_pos);
    } else { println!("Initialized Positions:\n  Start: {:?}\n  End: {:?}", start_pos, target_pos) }

    // BFS Implementation
    let mut to_explore: Vec<Position> = vec![start_pos];
    let mut visited: Vec<Position> = vec![];

    while !to_explore.is_empty() {
      let current = to_explore.pop().unwrap();

      if current.0 == target_pos.0 && current.1 == target_pos.1 {
        break;
      }

      let row = current.0;
      let col = current.1;
      if row > 0 { to_explore.insert(0, Position(row - 1, col)) } // left
      if row < height { to_explore.insert(0, Position(row + 1, col)) } // right
      if col > 0 { to_explore.insert(0, Position(row, col - 1)) } // up
      if col < width { to_explore.insert(0, Position(row, col + 1)) } // down
    }


  }
  
}