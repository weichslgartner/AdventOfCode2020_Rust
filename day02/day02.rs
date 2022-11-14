use std::fs;


struct PwRule {
    min_occ: usize,
    max_occ: usize,
    character: char,
    pw: String,
}

fn to_lines(input: &str) -> Vec<&str>{
    input
    .split("\n")
    .collect::<Vec<_>>()
}


fn parse_rule(input: &str) -> PwRule{
    let tokenz : Vec<_> = input.split(" ").collect();
    let occ_string : Vec<_> =  tokenz[0].split("-").collect();
    let character =  tokenz[1].chars().next().unwrap();
    let password= tokenz[2].trim();
    PwRule{min_occ: occ_string[0].parse::<usize>().unwrap(),
        max_occ: occ_string[1].parse::<usize>().unwrap(),character, pw: password.to_string()}
}

fn part1(rule: &PwRule) -> bool{
    let count = rule.pw.chars().filter(|x| *x==rule.character).count();
    count >= rule.min_occ && count <= rule.max_occ
}

fn part2(rule: &PwRule) -> bool{
    let to_match = rule.character as u8;
    (rule.pw.as_bytes()[rule.min_occ-1]==to_match) ^ (rule.pw.as_bytes()[rule.max_occ-1]==to_match)
}

fn main() {
    let file_name = "input_02.txt";
    let contents = fs::read_to_string(file_name).expect("File {} not found.");
    let lines = to_lines(&contents);
    let rules: Vec<PwRule> = lines.iter().map(|x| parse_rule(x)).collect();
    let part1 = rules.iter().filter(|&x| part1(x)).count();
    println!("Part 1: {}",part1);
    let part2 = rules.iter().filter(|&x| part2(x)).count();
    println!("Part 2: {}",part2);
}

