#[allow(unused)]
pub mod solutions {
  struct Assignment {
    start: u32,
    end: u32,
  }

  /// Day 4 Part 1 -- https://adventofcode.com/2022/day/4
  /// 
  /// Given a set of two ranges formatted (2-6,4-8), determine if either one
  /// is fully contained by the other. 
  /// Return the sum of ranges fully contained.
  pub fn find_fully_contained_assignments(assignments: &Vec<&str>) {
    let contained_assignments: u32 = assignments.iter().map(|assignment_pair| {
      let range_pairs: Vec<&str> = assignment_pair.split(',').collect();
      
      let pairs: Vec<Assignment> = range_pairs.iter().map(|range_pair| {
        let pair_split: Vec<u32> = range_pair
          .split('-')
          .map(|n| n.parse::<u32>().unwrap())
          .collect();
        return Assignment { start: pair_split[0], end: pair_split[1] };
      }).collect();

      if check_full_containment(&pairs[0], &pairs[1]) || check_full_containment(&pairs[1], &pairs[0]) {
        return 1
      } 
      return 0;
    }).into_iter().sum();

    println!("Fully contained assignments: {contained_assignments}")
  }

  /// Day 4 Part 2 -- https://adventofcode.com/2022/day/4/ 
  /// 
  /// Same as Part 1 but check for partially contained assignments instead
  /// of exclusively searching for completely contained assignments.
  pub fn find_partially_contained_assignments(assignments: &Vec<&str>) {
    let contained_assignments: u32 = assignments.iter().map(|assignment_pair| {
      let range_pairs: Vec<&str> = assignment_pair.split(',').collect();
      
      let pairs: Vec<Assignment> = range_pairs.iter().map(|range_pair| {
        let pair_split: Vec<u32> = range_pair
          .split('-')
          .map(|n| n.parse::<u32>().unwrap())
          .collect();
        return Assignment { start: pair_split[0], end: pair_split[1] };
      }).collect();

      if check_partial_containment(&pairs[0], &pairs[1]) || check_partial_containment(&pairs[1], &pairs[0]) {
        return 1
      } 
      return 0;
    }).into_iter().sum();

    println!("Partially contained assignments: {contained_assignments}")
  }

  fn check_full_containment(a1: &Assignment, a2: &Assignment) -> bool {
    if a1.start <= a2.start && a1.end >= a2.end { return true }
    return false
  }

  fn check_partial_containment(a1: &Assignment, a2: &Assignment) -> bool {
    if a1.end < a2.start || a1.start > a2.end { return false } 
    return true
  }
}