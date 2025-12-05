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
        5
    }

    fn get_year(&self) -> u16 {
        2025
    }
}

#[derive(Debug, Clone, PartialEq, Eq)] // Add traits as needed for your use case
struct Range {
    start: u64,
    end: u64,
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part A", Day.get_day());
    let mut result: u32 = 0;
    let mut split_index = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.trim().is_empty() {
            split_index = i;
            break;
        }
    }
    let (range_strings, id_strings) = lines.split_at(split_index);
    let mut ids: Vec<u64> = Vec::new();
    for id_str in id_strings {
        if id_str.trim().is_empty() {
            continue;
        }
        let id = id_str.trim().parse().unwrap();
        ids.push(id);
    }
    for id in ids{
        for range_str in range_strings {
            let parts: Vec<&str> = range_str.split('-').collect();
            if parts.len() != 2 {
                continue;
            }
            let start: u64 = parts[0].trim().parse().unwrap();
            let end: u64 = parts[1].trim().parse().unwrap();
            if id >= start && id <= end {
                result += 1;
                break;
            }
        }
    }
    println!("Result: {}", result);
    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part B", Day.get_day());
        let mut result: u64 = 0;
    let mut split_index = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.trim().is_empty() {
            split_index = i;
            break;
        }
    }
    let (range_strings, _) = lines.split_at(split_index);
    let mut ranges: Vec<Range> = Vec::new();
    for range_str in range_strings {
        if range_str.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            continue;
        }
        let start: u64 = parts[0].trim().parse().unwrap();
        let end: u64 = parts[1].trim().parse().unwrap();
        ranges.push(Range { start, end });
    }
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut collapsed_ranges: Vec<Range> = Vec::new();
    for range in ranges {
        if collapsed_ranges.is_empty() {
            collapsed_ranges.push(range);
        } else {
            let last_range = collapsed_ranges.last_mut().unwrap();
            if range.start <= last_range.end + 1 {
                if range.end > last_range.end {
                    last_range.end = range.end;
                }
            } else {
                collapsed_ranges.push(range);
            }
        }
    }
    for range in collapsed_ranges {
        result += range.end - range.start + 1;
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
