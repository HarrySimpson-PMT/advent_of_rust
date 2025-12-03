use core::num;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use tokio::io;
use tokio::io::AsyncReadExt;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 1, Part A");
    let ranges: Vec<String> = lines[0]
        .split(|c| c == ',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let mut result: i64 = 0;
    for range in ranges {
        //parge range into lower and upper i64 via '-'
        let bounds: Vec<&str> = range.split('-').collect();
        let lower: i64 = bounds[0].parse().unwrap();
        let upper: i64 = bounds[1].parse().unwrap();
        for num in lower..=upper {
            let num_as_string = num.to_string();
            if num_as_string.len() % 2 != 0 {
                continue;
            }
            let first_half = &num_as_string[..&num_as_string.len() / 2];
            let second_half = &num_as_string[&num_as_string.len() / 2..];
            if first_half == second_half {
                result += num;
            }
        }
    }
    println!("Result: {}", result);
    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 1, Part B");
     let ranges: Vec<String> = lines[0]
        .split(|c| c == ',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    let mut result: i64 = 0;
    for range in ranges {
        let bounds: Vec<&str> = range.split('-').collect();
        let lower: i64 = bounds[0].parse().unwrap();
        let upper: i64 = bounds[1].parse().unwrap();
        for num in lower..=upper {
            let num_as_string = num.to_string();
            for len in 1..=num_as_string.len() / 2 {
                let mut pattern_found = true;
                let pattern = &num_as_string[..len];
                let mut index = len;
                while index + len <= num_as_string.len() {
                    if &num_as_string[index..index + len] != pattern {
                        pattern_found = false;
                        break;
                    }
                    index += len;
                }
                if pattern_found && index == num_as_string.len() {
                    result += num;
                    break;
                }
            }
        }
    }
    println!("Result: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input_for_puzzle;
    use crate::Puzzle;
    fn get_day_name() -> String {
        let module_path = module_path!();
        let module_name = module_path.split("::").last().unwrap_or("Unknown");
        module_name.to_string().replace("day", "Day")
    }

    fn get_puzzle(part: char) -> Puzzle {
        let day = get_day_name().replace("Day", "").parse::<u8>().unwrap_or(1);
        match part {
            'A' => Puzzle::from_day_part(day, 'A'),
            'B' => Puzzle::from_day_part(day, 'B'),
            _ => panic!("Invalid part"),
        }
    }
}
