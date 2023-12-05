use anyhow::{anyhow, Context};
use std::fs;

mod day_01;

type Solver = fn(&str) -> Result<String, anyhow::Error>;

pub fn solve(day: u8, part: u8, file: Option<String>) -> Result<String, anyhow::Error> {
    let fn_list: Vec<Vec<Solver>> = vec![vec![day_01::part_1::solve, day_01::part_2::solve]];

    if !(1..=1).contains(&day) {
        return Err(anyhow!("invalid day: {}", day));
    }

    if !(1..=2).contains(&part) {
        return Err(anyhow!("invalid part: {}", part));
    }

    let file_path = get_file_path(day, part, file)?;

    let contents = read_file_to_string(&file_path)?;

    fn_list[day as usize - 1][part as usize - 1](&contents)
}

fn get_file_path(day: u8, _: u8, file: Option<String>) -> Result<String, anyhow::Error> {
    if let Some(existing_path) = file {
        return Ok(existing_path);
    }
    Ok(format!("input/day_{:02}/input.txt", day))
}

fn read_file_to_string(file_path: &str) -> Result<String, anyhow::Error> {
    fs::read_to_string(&file_path)
        .with_context(|| format!("could not read input file: {}", &file_path))
}
