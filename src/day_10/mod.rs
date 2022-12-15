#[allow(unused)]
pub mod solutions {
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

  fn check_strength(cycles: Vec<u32>, register_value: i64) -> Option<Vec<i64>> {
    let target_strengths: Vec<u32> = vec![20, 60, 100, 140, 180, 220]; 
    let cycles_that_matter = cycles.iter().filter(|n| target_strengths.contains(n)).collect::<Vec<&u32>>();
    // cycles_that_matter.iter().for_each(|cycle| println!("Cycle: {cycle}  Value: {register_value}  Str: {}", i64::from(**cycle) * (register_value)));
    // println!("Cycles: {cycles:?}  Cycle that matters: {cycles_that_matter:?}");
    if (cycles_that_matter.len() == 0) { 
      return None; 
    } else {
      println!("Cycles that matter! {cycles_that_matter:?}");
      println!("Register value: {register_value}");
      return Some(cycles_that_matter.iter().map(|cycle| i64::from(**cycle) * (register_value)).collect())
    }
  }
}