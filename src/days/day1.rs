use std::collections::HashSet;
use std::error::Error;

pub fn solve(input: String) -> Result<(String, String), Box<Error>> {
    Ok((solve_1(&input)?, solve_2(&input)?))
}

fn solve_1(input: &str) -> Result<String, Box<Error>> {
    let mut result: i64 = 0;
    for arg in input.trim().split("\n") {
        let number: i64 = arg.parse()?;
        result += number;
    }
    Ok(result.to_string())
}

fn solve_2(input: &str) -> Result<String, Box<Error>> {
    let mut freqs: HashSet<i64> = HashSet::new();
    let mut found: bool = false;
    let mut result: i64 = 0;

    while found == false {
        for arg in input.trim().split("\n") {
            let number: i64 = arg.parse()?;
            result += number;

            if freqs.contains(&result) {
                found = true;
                break;
            } else {
                freqs.insert(result);
            }
        }
    }

    Ok(result.to_string())
}
