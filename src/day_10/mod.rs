#[allow(unused)]
pub mod solutions {
    use std::thread::current;

  /// Day 10 Part 1 -- https://adventofcode.com/2022/day/10
  /// 
  /// Input is a series of instructions for a computer where something happens
  /// every single tick, or "cycle". There's one register total that starts
  /// with a value of 1. There are two instructions
  /// 
  /// addx <i>
  /// - Takes two cycles to complete action
  /// - On an integer <i> (can be positive or negative) to the register
  /// 
  /// noop
  /// - Takes one cycle to complete
  /// - No operation enacted
  /// 
  pub fn sum_six_signal_strengths(program: &Vec<&str>) {
    let mut cycle: u32 = 1;
    let mut register: i64 = 1;
    let mut important_cycle_strengths: Vec<i64> = Vec::new();

    for command in program {
      let args: Vec<&str> = command.split(" ").collect();
      match args[0] {
        "addx" => {
          let out = check_strength(vec!(cycle, cycle+1), register);
          if out.is_some() { important_cycle_strengths.append(&mut out.unwrap()); }
          cycle += 2;
          register += args[1].parse::<i64>().unwrap();
        },
        "noop" => {
          let out = check_strength(vec!(cycle), register);
          if out.is_some() { important_cycle_strengths.append(&mut out.unwrap()); }
          cycle += 1;
        },
        _ => panic!("Command not recognized!"),
      }
    }

    println!("Important Cycle Strengths: {:?}", important_cycle_strengths);
    println!("Cycle strength sum: {}", important_cycle_strengths.iter().sum::<i64>())
  }

  /// Day 10 Part 2 -- https://adventofcode.com/2022/day/10#part2
  /// 
  /// Given the same input, you must now use that input to represent
  /// a CRT display. The description is a bit complicated, read it via the link
  /// if you're interested, but the TL;DR: is print out the CRT display
  /// and the solution is the 8 capital letters contained within it.
  pub fn print_crt_display(program: &Vec<&str>) {
    let mut cycle: i32 = 1;
    let mut register: i32 = 1;
    let mut current_row: Vec<&str> = vec![];

    for command in program {
      let args: Vec<&str> = command.split(" ").collect();
      
      match args[0] {
        "addx" => {
          current_row.push(get_pixel_state(cycle, register));
          current_row.push(get_pixel_state(cycle + 1, register));
          cycle += 2;
          register += args[1].parse::<i32>().unwrap();
        },
        "noop" => {
          current_row.push(get_pixel_state(cycle, register));
          cycle += 1;
        },
        _ => panic!("Command not recognized!"),
      }

      if current_row.len() >= 40 { 
        println!("{:?}  Cycle: {cycle}", current_row[0..40].join(""));
        current_row.drain(0..40);
      }
    }
  }

  /// Helper function to check if a CRT signal is lit based on register pos + cycle
  fn get_pixel_state(cycle: i32, register: i32) -> &'static str {
    let crt_pos = (cycle - 1) % 40;
    if vec![crt_pos-1, crt_pos, crt_pos+1].contains(&register) {
      return "#"
    } else {
      return "."
    }
  }

  /// Helper function to check the strength of a set of cycles (Part 1)
  fn check_strength(cycles: Vec<u32>, register_value: i64) -> Option<Vec<i64>> {
    let target_strengths: Vec<u32> = vec![20, 60, 100, 140, 180, 220]; 
    let cycles_that_matter = cycles.iter().filter(|n| target_strengths.contains(n)).collect::<Vec<&u32>>();
    if (cycles_that_matter.len() == 0) { 
      return None; 
    } else {
      return Some(cycles_that_matter.iter().map(|cycle| i64::from(**cycle) * (register_value)).collect())
    }
  }
}