use crate::solver::DaySolver;
use tokio::io;

pub struct Day;

impl DaySolver for Day {
    async fn solve_a(&self, lines: &Vec<String>) -> io::Result<()> {
        solve_a(lines).await
    }

    async fn solve_b(&self, lines: &Vec<String>) -> io::Result<()> {
        solve_b(lines).await
    }

    fn get_day(&self) -> u8 {
        6
    }

    fn get_year(&self) -> u16 {
        2025
    }
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part A", Day.get_day());
    let ops_line = lines.last().unwrap().as_bytes();
    let mut ops = Vec::with_capacity(
        ops_line
            .iter()
            .filter(|&&b| b.is_ascii_whitespace())
            .count()
            + 1,
    );

    for &b in ops_line {
        if b == b'+' || b == b'*' {
            ops.push(b as char);
        }
    }
    let numeric_lines: Vec<Vec<u16>> = lines
        .iter()
        .take(lines.len().saturating_sub(1))
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u16>().unwrap())
                .collect()
        })
        .collect();

    let mut result: i64 = 0;
    for i in 0..ops.len() {
        match ops[i] {
            '+' => {
                let mut column_result: i64 = 0;
                for j in 0..numeric_lines.len() {
                    column_result += numeric_lines[j][i] as i64;
                }
                result += column_result;
            }
            '*' => {
                let mut column_result: i64 = 1;
                for j in 0..numeric_lines.len() {
                    column_result *= numeric_lines[j][i] as i64;
                }
                result += column_result;
            }
            _ => {
                println!("Unknown operation: {}", ops[i]);
            }
        }
    }
    println!("Result: {}", result);
    Ok(())
}


pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part B", Day.get_day());

    let ops_line = lines.last().unwrap().as_bytes();
    let mut ops = Vec::with_capacity(
        ops_line
            .iter()
            .filter(|&&b| b.is_ascii_whitespace())
            .count()
            + 1,
    );

    for &b in ops_line {
        if b == b'+' || b == b'*' {
            ops.push(b as char);
        }
    }


    let mut pos = 0;
    let mut result: i64 = 0;
    let mut sub_result: i64 = 0;
    let mut column = 0;
    let column_size = lines[0].len();

    loop {
        let column_number: i64 =
            lines
                .iter()
                .take(lines.len().saturating_sub(1))
                .fold(0, |acc, line| {
                    let Some(&b) = line.as_bytes().get(pos) else {
                        return acc;
                    };
                    if !b.is_ascii_digit() {
                        return acc;
                    }
                    acc * 10 + (b - b'0') as i64
                });
        if column_number == 0 {
            result += sub_result;
            column += 1;
            sub_result = 0;
        } else {
            match ops[column] {
                '+' => {
                    sub_result += column_number;
                }
                '*' => {
                    if sub_result == 0 {
                        sub_result = 1;
                    }
                    sub_result *= column_number;
                }
                _ => {
                    println!("Unknown operation: {}", ops[pos]);
                }
            }
        }
        pos += 1;
        if pos >= column_size {
            break;
        }
    }
    println!("Result: {}", result + sub_result);
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
