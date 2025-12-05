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
        1
    }

    fn get_year(&self) -> u16 {
        2025
    }
}


pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 1, Part A");
    let mut position = 50;
    let mut result = 0;
    //mod result by 100 for each rotation to see if % 100 == 0
    for line in lines {
        //parse line L14 or R12 where L should decrement and right should increment
        let (turn, dist) = line.split_at(1);
        let dist: i32 = dist.parse().unwrap();
        match turn {
            "L" => {
                position -= dist;
            }
            "R" => {
                position += dist;
            }
            _ => {
            }
        }
        if position % 100 == 0 {
            result += 1;
        }
    }
    println!("Final position: {}, Result: {}", position, result);

    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 1, Part B");
    let mut position = 50;
    let mut last_position = 50;
    let mut result = 0;
    for line in lines {
        let (turn, dist) = line.split_at(1);
        let dist: i32 = dist.parse().unwrap();
        match turn {
            "L" => {
                position -= dist;
                let last_hundreds = last_position / 100;
                let current_hundreds = position / 100;
                if last_hundreds == current_hundreds && position < 0 && last_position != 0 {
                    result += 1;
                }
                if last_hundreds != current_hundreds {
                    let extra = if last_position!=0 { 1 } else { 0 };
                    let diff = (current_hundreds - last_hundreds).abs() + extra;
                    result += diff;
                }
                if position == 0 {
                    result += 1;
                }
            }
            "R" => {
                position += dist;

                let last_hundreds = last_position / 100;
                let current_hundreds = position / 100;
                if last_hundreds != current_hundreds {
                    let diff = (current_hundreds - last_hundreds).abs();
                    result += diff;
                }
            }
            _ => {
                println!("Invalid turn direction: {}", turn);
            }
        }
        position = position % 100;
        if position < 0 {
            position += 100;
        }
        last_position = position;
    }

    println!("Final position: {}, Result: {}", position, result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input_for_puzzle;
    use crate::Puzzle;
    /// Determines the day name (e.g., "Day01") based on the module path
    fn get_day_name() -> String {
        let module_path = module_path!(); // e.g., "puzzles::day01"
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
