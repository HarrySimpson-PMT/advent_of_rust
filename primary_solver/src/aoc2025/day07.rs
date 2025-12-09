use crate::solver::DaySolver;
use std::collections::HashMap;
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
        7
    }

    fn get_year(&self) -> u16 {
        2025
    }
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part A", Day.get_day());
    let mut lines = lines.clone();
    let mut result: u32 = 0;
    for i in 0..lines.len() - 1 {
        for j in 0..lines[0].len() {
            let c = lines[i].chars().nth(j).unwrap();
            match c {
                'S' => {
                    lines[i + 1].replace_range(j..=j, "|");
                }
                '|' => {
                    let nc = lines[i + 1].as_bytes()[j] as char;
                    if nc == '^' {
                        result += 1;
                        if j > 0 {
                            lines[i + 1].replace_range(j - 1..=j - 1, "|");
                        }
                        if j < lines[0].len() - 1 {
                            lines[i + 1].replace_range(j + 1..=j + 1, "|");
                        }
                    } else {
                        lines[i + 1].replace_range(j..=j, "|");
                    }
                }
                _ => {}
            }
        }
    }
    println!("Result: {}", result);
    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part B", Day.get_day());
    let mut result: u64 = 0;
    let mut map: HashMap<(usize, usize), u64> = HashMap::new();
    let mut start_x = 0;
    let start_y = lines[0].find('S').unwrap();
    loop {
        let c = lines[start_x].as_bytes()[start_y] as char;
        if c == '^' {
            map.insert((start_x, start_y), 1);
            break;
        }
        start_x += 1;
    }
    for x in start_x..lines.len() {
        for y in 0..lines[0].len() {
            if !map.contains_key(&(x, y)) {
                continue;
            }
            let currnt_value = *map.get(&(x, y)).unwrap();
            if y > 0 {
                let mut nx = x + 1;
                loop {
                    let nc = lines[nx].as_bytes()[y - 1] as char;
                    if nc == '^' {
                        let entry = map.entry((nx, y - 1)).or_insert(0);
                        *entry += currnt_value;
                        break;
                    } else if nx + 1 >= lines.len() {
                        result += currnt_value as u64;
                        break;
                    }
                    nx += 1;
                }
            }
            if y + 1 < lines[0].len() {
                let mut nx = x + 1;
                loop {
                    let nc = lines[nx].as_bytes()[y + 1] as char;
                    if nc == '^' {
                        let entry = map.entry((nx, y + 1)).or_insert(0);
                        *entry += currnt_value;
                        break;
                    } else if nx + 1 >= lines.len() {
                        result += currnt_value as u64;
                        break;
                    }
                    nx += 1;
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
