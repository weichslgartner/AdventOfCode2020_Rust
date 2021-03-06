use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

fn parse_numbers(input: &str) -> HashSet<u32> {
    input
        .split("\n")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>()
}

fn part1(numbers: &HashSet<u32>, target: u32) -> Option<u32> {
    for x in numbers.iter() {
        let need = target - x;
        if numbers.contains(&need) {
            let result = x * need;
            return Some(result);
        }
    }
    None
}

fn part1_func(numbers: &HashSet<u32>, target: u32) -> Option<u32> {
    let res: Vec<_> = numbers
        .iter()
        .map(|x| target-x)
        .filter(|x|  
            numbers.contains(x)
        )
        .map(|x| (target-x)*x)
        .collect();
    if res.len() > 0{
        return Some(res[0]);
    }
    None
    
}

fn part2(numbers: &HashSet<u32>, target: u32) -> Option<u32> {
    for x in numbers.iter() {
        for y in numbers.iter() {
            if x + y < target {
                let need = target - x - y;
                if numbers.contains(&need) {
                    return Some(x * need * y);
                }
            }
        }
    }
    None
}

fn part2_func(numbers: &HashSet<u32>, target: u32) -> Option<u32> {
    let res: Vec<_> = numbers
    .iter()
    .copied()
    .combinations(2)
    .map(|x| target-x[0]-x[1])
    .filter(|x|  
        numbers.contains(x)
    )
    .map(|x| (target-x)*x)
    .collect();
if res.len() > 0{
    return Some(res[0]);
}
None
}

fn main() {
    let file_name = "input_01.txt";
    let contents = fs::read_to_string(file_name).expect("File {} not found.");
    let numbers = parse_numbers(&contents);
    let target = 2020;
    print!("part1: ");
    match part1(&numbers, target) {
        None => print!("no solution\n"),
        Some(x) => print!(" {}\n", x),
    }
    print!("part1 (functional): ");
    match part1_func(&numbers, target) {
        None => print!("no solution\n"),
        Some(x) => print!(" {}\n", x),
    }
    print!("part2: ");
    match part2(&numbers, target) {
        None => print!("no solution\n"),
        Some(x) => print!(" {}\n", x),
    }
}
