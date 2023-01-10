#[allow(unused)]
pub mod solutions {
  use std::time::{SystemTime, Duration};

  #[derive(Debug, Default, Clone, PartialEq)]
  enum Operation {
    #[default]
    Undefined,
    
    Multiplication,
    Addition,
  }

  #[derive(Debug, Default, Clone)] 
  struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    operation_amount: usize, // usize::MAX means multiply by self
    test_amount: usize,
    true_monkey_target: usize,
    false_monkey_target: usize, 
    inspection_count: usize,
  }

  impl Monkey {
    fn add_item(&mut self, item: usize) {
      let mut new_items = Vec::from(self.items.clone());
      new_items.push(item);
      self.items = new_items;
    }

    fn clear_items(&mut self) {
      self.items.clear();
    }
  }


  /// Day 11 Part 1 -- https://adventofcode.com/2022/day/11
  /// 
  /// Input is a series of monkeys, items, and the relationships
  /// between them. Each monkey will have a test with an operation, 
  /// a true statement, and a false statement. If the statement 
  /// evaluates to true, activate the true statement otherwise activate
  /// the false statement.
  /// 
  /// Count the total number of times the monkeys evaluate the statements 
  /// the course of 20 cycles. The total "monkey business" is the product (*)
  /// of the two most active monkeys over the duration.
  pub fn calculate_monkey_business(monkey_notes: &Vec<&str>) {
    let mut monkeys = parse_monkeys(monkey_notes);
    for (i, monkey) in monkeys.iter().enumerate() { println!("Monkey {i}: {:?}", monkey) }

    for _ in 0..20 {
      for i in 0..monkeys.len() {
        let mut item_set =  monkeys[i].items.clone();
        monkeys[i].clear_items();
        
        while item_set.len() > 0 {
          let current = item_set.pop().unwrap();
          monkeys[i].inspection_count += 1;
          let (next_location, next_value) = get_next_item_location(current, &monkeys[i], false);
          monkeys[next_location].add_item(next_value);
        }
      }
    }

    let specific_monkey_business = calculate_specific_monkey_business(&monkeys);
    println!("Monkey business for top 2 apes: {specific_monkey_business}");
  }

/// Part 2 is the same as part 1 except over the course of 10,000 cycles instead of 20
// pub fn calculate_big_monkey_business(monkey_notes: &Vec<&str>) {
//   let mut monkeys = parse_monkeys(monkey_notes);
//   let start_timestamp = SystemTime::now();
  
//   for _ in 0..10 {
//     for i in 0..monkeys.len() {
//       let mut item_set =  monkeys[i].items.clone();
//       monkeys[i].clear_items();
      
//       while item_set.len() > 0 {
//         let current = item_set.pop().unwrap();
//         monkeys[i].inspection_count += 1;
//         let (next_location, next_value) = get_next_item_location(current, &monkeys[i], false);
//         monkeys[next_location].add_item(next_value);
//       }

//       debug_monkeys(&monkeys, i);
//     }
//   }
//   println!("Elapsed time: {:?}", start_timestamp.elapsed());

//   let specific_monkey_business = calculate_specific_monkey_business(&monkeys);
//   println!("Monkey business for top 2 apes: {specific_monkey_business}");
// }

  fn debug_monkeys(monkeys: &Vec<Monkey>, round: usize) {
    println!("\n Round {round}: ");
    for (i, monkey) in  monkeys.clone().iter().enumerate() {
      println!("  Monkey {i}: {:?}", monkey.items);
    }
  }

  fn calculate_specific_monkey_business(monkeys: &Vec<Monkey>) -> usize {
    let mut inspection_counts: Vec<usize> = monkeys.iter()
      .map(|m| m.inspection_count)
      .collect::<Vec<usize>>();
  
    inspection_counts.sort();

    return inspection_counts.iter()
      .rev()
      .take(2)
      .product();
  }

  /// Pulls the tail word off a sentence by spaces
  fn get_last_word(s: &str) -> &str {
    s.split(" ")
      .collect::<Vec<&str>>()
      .pop()
      .unwrap()
  }

  /// Returns a tuple of (monkey_location, item_value)
  fn get_next_item_location(item: usize, monkey: &Monkey, anxiety_calms: bool) -> (usize, usize) {
    let mut new_item_value;
    match monkey.operation {
      Operation::Addition => new_item_value = (item + monkey.operation_amount),
      Operation::Multiplication => {
        match monkey.operation_amount == usize::MAX { 
          true => new_item_value = (item * item),
          false =>  new_item_value = (item * monkey.operation_amount),
        }
      },
      _ => panic!("Illegal operation request for monkey: {:?}", monkey)
    }

    if anxiety_calms { new_item_value = new_item_value / 3; }
    

    match new_item_value % monkey.test_amount == 0 { 
      true =>  return (monkey.true_monkey_target, new_item_value),
      false => return (monkey.false_monkey_target, new_item_value),
    } 
  }

  /// Parses monkey data from input vec of strings into a vec of monkeys
  fn parse_monkeys(monkey_notes: &Vec<&str>) -> Vec<Monkey> {
    let mut monkey_number = 0;
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_template = Monkey::default();

    for line in monkey_notes {
      if line.len() == 0 {
        monkey_number += 1;
        monkeys.push(monkey_template.clone());
        monkey_template = Monkey::default();
        continue;
      }

      let split: Vec<&str> = line.trim().split(":").collect();
      let info_type = split[0];

      if info_type.split(" ").collect::<Vec<&str>>()[0] == "Monkey" { continue; }

      let data = split[1].trim();
      match info_type {
        "Starting items" => {
          monkey_template.items = data
            .trim()
            .split(", ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        },
        "Operation" => {
          if data.contains("*") { monkey_template.operation = Operation::Multiplication }
          else if data.contains("+") { monkey_template.operation = Operation::Addition }
          else { panic!("Unable to define monkey operation for string: {data}") }

          let amount = get_last_word(data);
          if amount == "old" { monkey_template.operation_amount = usize::MAX }
          else {monkey_template.operation_amount = amount.parse::<usize>().unwrap() }
        },
        "Test" => monkey_template.test_amount = get_last_word(data).parse::<usize>().unwrap(),
        "If true" => monkey_template.true_monkey_target = get_last_word(data).parse::<usize>().unwrap(),
        "If false" => monkey_template.false_monkey_target = get_last_word(data).parse::<usize>().unwrap(),
        _ => panic!("Unexpected input type: '{}'", data),
      }
    } 

    return monkeys;
  }

}