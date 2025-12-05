use tokio::io;
use crate::solver::DaySolver;

pub struct Day;

impl DaySolver for Day {
    async fn solve_a(&self, lines: &Vec<String>) -> io::Result<()> {
        solve_a(lines).await
    }

    async fn solve_b(&self, lines: &Vec<String>) -> io::Result<()> {
        solve_b(lines).await
    }

    fn get_day(&self) -> u8{
        3
    }

    fn get_year(&self) -> u16 {
        2025
    }
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 3, Part A");
    let mut stack: Vec<u32> = Vec::new();
    let mut result: i64 = 0;
    for line in lines{
        for i in 0..line.len()-1{        
            if let Some(digit) = line.chars().nth(i).and_then(|c| c.to_digit(10)) {
                while !stack.is_empty() && *stack.last().unwrap() < digit {
                    stack.pop();
                }
                if stack.len() >= 2 {
                    continue;
                }else {
                    stack.push(digit);
                }
            }
        }
        let last_char = line.chars().last().and_then(|c| c.to_digit(10));
        if stack.len() < 2 {
            stack.push(last_char.unwrap_or(0));
        } else if *stack.last().unwrap_or(&0) < last_char.unwrap_or(0) {
            stack.pop();
            stack.push(last_char.unwrap_or(0));
        }
        //revers out the stack and add its value to result
        let mut current: String = String::new();
        while !stack.is_empty() {
            current.insert_str(0, &stack.pop().unwrap().to_string());
        }
        result += current.parse::<i64>().unwrap();
    }
    println!("Result: {}", result);
    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 3, Part B");
    let mut stack: Vec<u32> = Vec::new();
    let mut result: i64 = 0;
    for line in lines{
        for i in 0..line.len(){        
            if let Some(digit) = line.chars().nth(i).and_then(|c| c.to_digit(10)) {
                while !stack.is_empty() && *stack.last().unwrap() < digit && stack.len()+line.len()-i >12 {
                    stack.pop();
                }

                if stack.len() >= 12 {
                    continue;
                }else {
                    stack.push(digit);
                }
            }
        }
        let mut current: String = String::new();
        while !stack.is_empty() {
            current.insert_str(0, &stack.pop().unwrap().to_string());
        }
        result += current.parse::<i64>().unwrap();
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
