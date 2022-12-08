#[allow(unused)]
pub mod solutions {
  use trees::Node;

  /// Day 7 Part 1 -- https://adventofcode.com/2022/day/7
  /// 
  /// Your input is a CLI dump. Anything marked with $ is a command,
  /// anything otherwise is a print output. Different commands do different things,
  /// as denoted below. Find the sum of all directories with a size of 100,000 
  /// units or less.
  /// 
  /// | Command  | Description |
  /// |----------|-------------|
  /// | $ cd <x> | Move into directory <x> |
  /// | $ cd ..  | Move up into parent directory |
  /// | $ cd /   | Move to root directory |
  /// | $ ls     | Print all the files in the current directory |
  /// 
  /// Example output at demo root:
  /// ```console
  /// $ ls
  /// dir a            <-- This means there's a directory here named "a"
  /// 14848514 b.txt   <-- This means there's a file here named "b.txt" with size 14848514
  /// ```
  pub fn find_sum_dirs_under_100000(cli_dump: &Vec<&str>) {
    let root = Node { 
      parent: null,
      children: vec![], 
      files: vec![] };

    let mut current: &Node = &root;
    for line in cli_dump {
      let head = &line[0..4];
      let command: Vec<&str> = line.split(" ").collect();
      

      match head {
        "$ cd" => {
          println!("{}: change dir to {}", head, command[2]);
          if command[2] == "/" { current = &root; }
          else {
            let new_node = Node {
              
            }
          }
        },
        "$ ls" =>  {
          println!("{}: listing dir", head)
        },
        "dir " => {
          println!("{}: new dir named '{}'", head, command[1])
        },
        _ => {
          println!("{}: new file named {} of size {}", head, command[1], command[0])
        },
      }
      // println!("Split command: {:?}", command);
    }
  }
}