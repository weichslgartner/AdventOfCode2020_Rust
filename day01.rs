use std::fs;
use std::collections::HashSet;

pub fn parse_numbers(input: &str) -> HashSet<u32> {
    input
      .split("\n")
      .map(|s| s.parse::<u32>().unwrap())
      .collect::<HashSet<u32>>()
  }

fn part1(numbers: &HashSet<u32>, target: u32) -> u32{
    for x in numbers.iter(){
        let need = target - x;
        if numbers.contains(&need){
            return x * need;
            
        }
    }
    return 0;
}

fn part2(numbers: &HashSet<u32>, target: u32) -> u32{
    for x in numbers.iter(){
        for y in numbers.iter(){
            if x+y < target{
                let need = target - x-y;
                if numbers.contains(&need){
                    return x * need * y;
                }
            }
        }
    }
    return 0;
}


fn main() {
    let file_name = "input_01.txt";
    let contents = fs::read_to_string(file_name)
    .expect("File {} not found.");
    let numbers = parse_numbers(&contents);
    let target = 2020;
    print!("part1: {}\n", part1(&numbers,target));
    print!("part2: {}\n", part2(&numbers,target));
}
