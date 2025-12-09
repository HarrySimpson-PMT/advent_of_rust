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
    let result = 0;
   
    println!("Result: {}", result);
    Ok(())
}


pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part B", Day.get_day());
    let result = 0;

   
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
