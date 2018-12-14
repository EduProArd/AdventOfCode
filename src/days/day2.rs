use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

pub fn solve(input: String) -> Result<(String, String), Box<Error>> {
    Ok((solve_1(&input)?, solve_2(&input)?))
}

fn solve_1(input: &str) -> Result<String, Box<Error>> {
    let mut result: i64 = 0;
    let mut two: i64 = 0;
    let mut three: i64 = 0;

    for arg in input.trim().split("\n") {
        let mut map: HashMap<char, u8> = HashMap::new();
        for c in arg.chars() {
            if map.contains_key(&c) {
                *map.get_mut(&c).unwrap() += 1;
            } else {
                map.insert(c, 1);
            }
        }

        if map.values().any(|&a| a == 2) {
            two += 1;
        }
        if map.values().any(|&a| a == 3) {
            three += 1;
        }
    }
    result += three * two;
    Ok(result.to_string())
}

fn solve_2(input: &str) -> Result<String, Box<Error>> {
    let mut result = String::new();
    let mut first = String::new();
    let mut second = String::new();

    let mut set: HashSet<&str> = HashSet::new();
    for arg in input.trim().split("\n") {
        let mut filter_set = set
            .iter()
            .filter(|&a| is_diff_between(a, arg, 1))
            .collect::<Vec<&&str>>();

        if filter_set.is_empty() {
            set.insert(&arg);
        } else {
            first = filter_set[0].to_string();
            second = arg.to_string();
            break;
        }
    }

    for index in 0..first.len() {
        if first.chars().nth(index) == second.chars().nth(index) {
            result.push(first.chars().nth(index).unwrap());
        }
    }
    Ok(result.to_string())
}

fn is_diff_between(first: &str, second: &str, min_diff: u8) -> bool {
    let mut diffs: u8 = 0;
    for index in 0..first.len() {
        if first.chars().nth(index) != second.chars().nth(index) {
            diffs += 1;
        }
    }
    if diffs <= min_diff {
        return true;
    }
    return false;
}
