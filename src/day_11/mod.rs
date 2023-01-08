#[allow(unused)]
pub mod solutions {
  #[derive(Debug, Default, Clone, PartialEq)]
  enum Operation {
    #[default]
    Undefined,
    
    Multiplication,
    Addition,
  }

  #[derive(Debug, Default, Clone, )] 
  struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    operation_amount: usize, //usize::MAX means multiply by self
    test_amount: usize,
    true_monkey_target: usize,
    false_monkey_target: usize, 
  }

  impl Monkey {
    fn setItem(&mut self, item: usize) {
      let mut new_items = Vec::from(&self.items);
      new_items.push(item);
      self.items = new_items;
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
    println!("Monkeys parsed");
    for (i, monkey) in monkeys.iter().enumerate() { println!("Monkey {i}: {:?}", monkey) }

    for i in 0..2 {
      debug_monkeys(&monkeys, i);

      let mut next_round_monkeys = monkeys.clone()
        .iter()
        .map(|monkey| { 
          let mut new_monkey = monkey.clone();
          new_monkey.items.clear();
          return new_monkey;
        })
        .collect::<Vec<Monkey>>();

      for i in 0..monkeys.len() {
        for item in &monkeys[i].items {
          println!("\nMonkey {i} inspected item {item}");
          let (next_location, next_value) = get_next_item_location(*item, &monkeys[i]);
          let mut updated_items = next_round_monkeys[next_location].items.clone();
          updated_items.push(next_value);

          monkeys[next_location].items = updated_items;
        }
      }

    }
  }

  fn debug_monkeys(monkeys: &Vec<Monkey>, round: usize) {
    println!("\n Round {round}: ");
    for (i, monkey) in  monkeys.clone().iter().enumerate() {
      println!("Monkey {i}: {:?}", monkey.items);
    }
  }

  fn get_last_word(s: &str) -> &str {
    s.split(" ")
      .collect::<Vec<&str>>()
      .pop()
      .unwrap()
  }

  // Returns a tuple of (monkey_location, item_value)
  fn get_next_item_location(item: usize, monkey: &Monkey) -> (usize, usize) {
    let mut new_item_value;
    match monkey.operation {
      Operation::Addition => {
        new_item_value = (item + monkey.operation_amount) / 3;
        println!("Worry level increased by {}", monkey.operation_amount)
      },
      Operation::Multiplication => {
        if monkey.operation_amount == usize::MAX {
          new_item_value = (item * item) / 3;
          println!("Worry level increased by {} times", item)
        } else {
          new_item_value = (item * monkey.operation_amount) / 3;
          println!("Worry level increased by {} times", monkey.operation_amount)
        }
      },
      _ => panic!("Illegal operation request for monkey: {:?}", monkey)
    }

    if new_item_value % monkey.test_amount == 0 {
      println!("Throwing to monkey {} with new value {}", monkey.true_monkey_target, new_item_value);
      return (monkey.true_monkey_target, new_item_value)
    } else {
      println!("Throwing to monkey {} with new value {}", monkey.false_monkey_target, new_item_value);
      return (monkey.false_monkey_target, new_item_value)
    }
  }

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
      let data = split[1].trim();

      if info_type.split(" ").collect::<Vec<&str>>()[0] == "Monkey" {
        continue;
      }

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
        "Test" => {
          monkey_template.test_amount = get_last_word(data).parse::<usize>().unwrap();
        },
        "If true" => {
          monkey_template.true_monkey_target = get_last_word(data).parse::<usize>().unwrap();
        },
        "If false" => {
          monkey_template.false_monkey_target = get_last_word(data).parse::<usize>().unwrap();
        },
        _ => panic!("Unexpected input type: '{}'", data),
      }
    } 

    return monkeys;
  }

}