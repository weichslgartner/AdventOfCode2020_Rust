use std::collections::HashSet;

type CheckFn = fn(&str) -> bool;

const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const FIELD2FUN: [(&str, CheckFn); 7] = [
    ("byr", check_byr),
    ("iyr", check_iyr),
    ("eyr", check_eyr),
    ("hgt", check_hgt),
    ("hcl", check_hcl),
    ("ecl", check_ecl),
    ("pid", check_pid),
];

fn main() {
    let entries = parse_input();
    let c = entries.iter().filter(|x| test_passport_p1(x)).count();
    println!("Part 1: {c}");
    let c = entries.iter().filter(|x| test_passport_p2(x)).count();
    println!("Part 2: {c}");
}

fn parse_input() -> Vec<String> {
    include_str!("../input_04.txt")
        .split("\n\n")
        .map(|e| e.replace('\n', " "))
        .collect::<Vec<_>>()
}

fn test_passport_p1(passport: &str) -> bool {
    FIELDS.into_iter().all(|x| passport.contains(x))
}

fn test_passport_p2(passport: &str) -> bool {
    FIELD2FUN
        .into_iter()
        .all(|(key, fun)| check_field(passport, key, fun))
}

fn check_field(passport: &str, key: &str, fun: CheckFn) -> bool {
    if !passport.contains(key) {
        return false;
    }
    let tokenz: Vec<(&str, &str)> = passport
        .split(' ')
        .filter(|x| x.contains(key))
        .map(|x| x.rsplit_once(':').unwrap())
        .collect();
    fun(tokenz.first().unwrap().1)
}

fn check_year(year: &str, min: u32, max: u32) -> bool {
    let year = year.parse::<u32>().unwrap();
    year >= min && year <= max
}

fn check_hgt(token: &str) -> bool {
    if token.contains("in") {
        let height: u32 = token[0..token.len() - 2].parse().unwrap();
        return (59..=76).contains(&height);
    }
    if token.contains("cm") {
        let height: u32 = token[0..token.len() - 2].parse().unwrap();
        return (150..=193).contains(&height);
    }
    false
}

fn check_hcl(token: &str) -> bool {
    const HAIR_LEN: usize = 7;
    if (token.len() != HAIR_LEN) || !token.starts_with('#') {
        return false;
    }
    token.chars().filter(|x| x.is_ascii_hexdigit()).count() == HAIR_LEN - 1
}

fn check_ecl(token: &str) -> bool {
    if token.len() != 3 {
        return false;
    }
    let valid_ec = HashSet::from(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]);
    valid_ec.contains(token)
}

fn check_pid(token: &str) -> bool {
    const ID_LEN: usize = 9;
    if token.len() != ID_LEN {
        return false;
    }
    token.chars().filter(|x| x.is_ascii_digit()).count() == ID_LEN
}

fn check_byr(token: &str) -> bool {
    check_year(token, 1920, 2002)
}

fn check_iyr(token: &str) -> bool {
    check_year(token, 2010, 2020)
}

fn check_eyr(token: &str) -> bool {
    check_year(token, 2020, 2030)
}
