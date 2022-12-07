pub mod solutions {
  pub fn find_rearrangement_message(puzzle_input: &Vec<&str>) {
    let mut stacks = parse_initial_stacks(puzzle_input);
    let starting_index = calculate_stack_height(puzzle_input) + 2;

    println!("Starting index: {}", puzzle_input[starting_index]);
    for instruction_index in starting_index..puzzle_input.len() {
      let instructions: Vec<&str> = puzzle_input[instruction_index]
        .trim()
        .split(' ')
        .collect();
      println!("Instruction: {instructions:?}");
      println!("Len:         {}", instructions.len());
      let amount = instructions[1].parse::<usize>().unwrap();
      let source = instructions[3].parse::<usize>().unwrap() - 1;
      let target = instructions[5].parse::<usize>().unwrap() - 1;
      for _ in 0..amount {
        let data = stacks[source].pop().unwrap();
        stacks[target].push(data);
      }
      println!("Stacks Updated: {:?}", stacks)
    }

    let mut secret_message = "".to_owned();
    stacks.iter().for_each(|s| secret_message.push_str(s.last().unwrap()));

    println!("Secret message decrypted: {secret_message}");

  }

  fn parse_initial_stacks(input: &Vec<&str>) -> Vec<Vec<String>>{
    let max_stack_height = calculate_stack_height(&input);

    // Initialize stacks
    let total_stacks = input[0].len() / 4;
    let mut stacks: Vec<Vec<String>> = vec![];
    for _ in 0..total_stacks { stacks.push(vec![]); }

    // Populate stacks
    for i in (0..max_stack_height).rev() {
      let cleaned_string = input[i].replace(&['\r'][..], "");
      let mut j = 1;
      while j < cleaned_string.len() {
        let ch = cleaned_string.get(j..j+1).unwrap();
        if String::from(ch) != String::from(' ') { stacks[(j-1)/4].push(String::from(ch)); }
        j += 4;
      }
    }

    println!("Initial Stacks: {stacks:?}");
    return stacks;
  }

  fn calculate_stack_height(input: &Vec<&str>) -> usize {
    let mut max_stack_height = 0;
    for i in 0..input.len() {
      if input[i].len() == 1 { return i - 1 }
    }
    panic!("Stack height not found")
  }
}