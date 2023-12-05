use anyhow::anyhow;

pub(crate) fn solve(input: &str) -> Result<String, anyhow::Error> {
    let mut res = 0;
    for line in input.lines() {
        res += get_calibration_value(line)?;
    }
    Ok(res.to_string())
}

fn get_calibration_value(input: &str) -> Result<u32, anyhow::Error> {
    let first_digit = match input.chars().find(|c| c.is_ascii_digit()) {
        Some(c) => c.to_digit(10).unwrap(),
        None => return Err(anyhow!("no ascii digits in input line")),
    };

    let second_digit = match input.chars().rev().find(|c| c.is_ascii_digit()) {
        Some(c) => c.to_digit(10).unwrap(),
        None => return Err(anyhow!("no ascii digits in input line")),
    };

    Ok((first_digit * 10) + second_digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_calibration_value() {
        let input = "1abc2";
        let res = get_calibration_value(input).expect("should have a digit");
        assert_eq!(12, res);

        let input = "pqr3stu8vwx";
        let res = get_calibration_value(input).expect("should have a digit");
        assert_eq!(38, res);

        let input = "a1b2c3d4e5f";
        let res = get_calibration_value(input).expect("should have a digit");
        assert_eq!(15, res);
    }

    #[test]
    fn test_no_digit() {
        let input = "asdf";
        let res = get_calibration_value(input);
        assert!(res.is_err());
    }

    #[test]
    fn test_solve() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let res = solve(input).expect("should have valid input");
        assert_eq!("142", res);
    }
}
