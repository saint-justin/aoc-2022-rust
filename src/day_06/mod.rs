#[allow(unused)]
pub mod solutions {
    use std::collections::HashSet;

  /// Day 6 Part 1 -- https://adventofcode.com/2022/day/6
  /// 
  /// Given an input thats a random array of characters, find the 
  /// first character in a 4-string marker that denotes the start of 
  /// a radio signal. A given market is a 4-char string composed of completely
  /// unique characters. Find the position of the first character of the first market.
  pub fn characters_before_start_of_packet(signal_string: &Vec<&str>) {
    let signal = String::from(signal_string[0]);
    for i in 0..signal.len() - 3 {
      let mut set: HashSet<&char> = HashSet::new();
      let slice: Vec<char> = signal[i..i+4].chars().collect();
      slice.iter().for_each(|ch| { set.insert(ch); });

      if set.len() == 4 { 
        println!("Packet start marker found at position {}: {:?}", i+4, slice);
        return;
      }
    }
    panic!("Signal init not found!");
  }

  /// Day 6 Part 2 -- https://adventofcode.com/2022/day/6#part2
  /// 
  /// Same as above, except for a 14 character entry instead of 4
  pub fn characters_before_start_of_packet_big(signal_string: &Vec<&str>) {
    let signal = String::from(signal_string[0]);
    for i in 0..signal.len() - 13 {
      let mut set: HashSet<&char> = HashSet::new();
      let slice: Vec<char> = signal[i..i+14].chars().collect();
      slice.iter().for_each(|ch| { set.insert(ch); });

      if set.len() == 14 { 
        println!("Packet start marker found at position {}: {:?}", i+14, slice);
        return;
      }
    }
    panic!("Signal init not found!");
  }
}