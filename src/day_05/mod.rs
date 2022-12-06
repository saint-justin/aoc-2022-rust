pub mod solutions {
  pub fn find_rearrangement_message(puzzle_input: &Vec<&str>) {
    let initial_stacks = parse_initial_stacks(puzzle_input);
  }

  fn parse_initial_stacks(input: &Vec<&str>) {
    let mut max_stack_height = 0;
    for i in 0..input.len() {
      if input[i].len() == 1 { 
        max_stack_height = i - 1;
        break;
      }
    }

    let total_stacks = input[0].len() / 4;
    let mut stacks: Vec<Vec<&str>> = vec![];
    for _i in 0..total_stacks { stacks.push(vec![]); }

    println!("Stacks: {stacks:?}");

    for i in (0..max_stack_height).rev() {
      let row = input[i].replace(&['[',']'][..], "");
      let split: Vec<String> = row.split(' ').map(|s| String::from(s)).collect();
      split.iter()
        .enumerate()
        .for_each(|(i, entry)| stacks[i].push(&entry));
    }

    println!("Stacks: {stacks:?}");

  }
}