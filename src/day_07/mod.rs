#[allow(unused)]
pub mod solutions {
  use std::collections::HashMap;

  #[derive(Debug, Clone)]
  struct File {
    name: String,
    size: u32,
  }

  #[derive(Debug, Clone)]
  struct Directory<'a> {
    parent: &'a String,
    name: String,
    dirs: Vec<String>, // stores unique names of dirs instead of refs
    files: Vec<File>,
  }

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
    let mut directory: HashMap<String, Box<Directory>> = HashMap::new();
    let mut current: Box<String> = Box::from("/".to_owned());

    let mut root_dir = Directory {
      parent: &"".to_owned(),
      name: "/".to_owned(),
      dirs: vec![],
      files: vec![],
    };

    directory.insert(String::from("/"), Box::from(root_dir));

    for line in cli_dump {
      let head = &line[0..4];
      let command: Vec<&str> = line.split(" ").collect();
      let active_directory = String::from(&current.to_string());
      
      match head {
        "$ cd" => {
          println!("{}: change dir to {}", head, command[2]);
          current = Box::from(command[2].to_owned());
          continue;
        },
        "$ ls" =>  {
          println!("{}: listing dir", head);
          continue;
        },
        "dir " => {
          println!("{}: new dir named '{}'", head, command[1]);
          let mut current_dir = directory.get(&active_directory).unwrap().clone();
          let new_dir = Directory {
            parent: &current,
            name: command[1].to_owned(),
            dirs: vec![],
            files: vec![],
          };

          current_dir.dirs.push(command[1].to_owned());
          directory.insert(String::from(&current_dir.name), current_dir);
          directory.insert(String::from(&new_dir.name), Box::from(new_dir));
          continue;
        },
        _ => {
          println!("{}: new file named {} of size {}", head, command[1], command[0]);
          let mut current_dir = directory.get(&active_directory).unwrap().clone();
          let new_file = File {
            name: command[1].to_owned(),
            size: command[0].parse::<u32>().unwrap(),
          };
          current_dir.files.push(new_file);
          directory.insert(active_directory, current_dir);
          continue;
        },
      }
    }
  }
}