use anyhow::anyhow;
use std::collections::HashMap;

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let mut res = 0;
    for line in input.lines() {
        res += get_calibration_value(line)?;
    }
    Ok(res.to_string())
}

fn get_calibration_value(input: &str) -> Result<u32, anyhow::Error> {
    let first_digit = find_first(input)?.ok_or(anyhow!("no digits found"))?;
    let second_digit = find_second(input)?.ok_or(anyhow!("no digits found"))?;

    Ok((first_digit * 10) + second_digit)
}

fn find_first(input: &str) -> Result<Option<u32>, anyhow::Error> {
    let mut found = HashMap::new();
    let text = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    for i in 1..10 {
        if let Some(idx) = input.find(text[i - 1]) {
            found.insert(i, idx);
        }
        if let Some(idx) = input.find(digits[i - 1]) {
            if let Some(curr_idx) = found.get(&i) {
                if idx < *curr_idx {
                    found.insert(i, idx);
                }
            } else {
                found.insert(i, idx);
            }
        }
    }

    match found.iter().min_by(|a, b| a.1.cmp(b.1)) {
        Some(d) => Ok(Some(*d.0 as u32)),
        None => Ok(None),
    }
}

fn find_second(input: &str) -> Result<Option<u32>, anyhow::Error> {
    let mut found = HashMap::new();
    let text = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    for i in 1..10 {
        if let Some(idx) = input.rfind(text[i - 1]) {
            found.insert(i, idx);
        }
        if let Some(idx) = input.rfind(digits[i - 1]) {
            if let Some(curr_idx) = found.get(&i) {
                if idx > *curr_idx {
                    found.insert(i, idx);
                }
            } else {
                found.insert(i, idx);
            }
        }
    }

    match found.iter().max_by(|a, b| a.1.cmp(b.1)) {
        Some(d) => Ok(Some(*d.0 as u32)),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_calibration_value() {
        let input = "two1nine";
        let res = get_calibration_value(input).expect("should have a digit");
        assert_eq!(29, res);

        let input = "eightwothree";
        let res = get_calibration_value(input).expect("should have a digit");
        assert_eq!(83, res);

        let input = "abcone2threexyz";
        let res = get_calibration_value(input).expect("should have a digit");
        assert_eq!(13, res);

        let input = "three";
        let res = get_calibration_value(input).expect("should have a digit");
        assert_eq!(33, res);
    }

    #[test]
    fn test_no_digit() {
        let input = "asdfasdfasdf";
        let res = get_calibration_value(input);
        assert!(res.is_err());
    }

    #[test]
    fn test_solve() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let res = solve(input).expect("should have valid input");
        assert_eq!("281", res);
    }
}
