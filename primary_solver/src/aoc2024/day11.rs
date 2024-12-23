use tokio::io;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 11, Part A");

    let mut stones: Vec<u64> = lines[0]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    for _ in 0..25 {
        stones = blink(&stones);
    }

    println!("Number of stones after 25 blinks: {}", stones.len());

    Ok(())
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut new_stones = Vec::new();

    for &stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else if stone.to_string().len() % 2 == 0 {
            let digits = stone.to_string();
            let mid = digits.len() / 2;
            let left = digits[..mid].parse::<u64>().unwrap();
            let right = digits[mid..].parse::<u64>().unwrap();
            new_stones.push(left);
            new_stones.push(right);
        } else {
            new_stones.push(stone * 2024);
        }
    }


    new_stones
}

use std::collections::HashMap;

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 11, Part B");

    let initial_stones: Vec<u64> = lines[0]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut memo: HashMap<(u64, usize), usize> = HashMap::new();

    let total_stones: usize = initial_stones
        .iter()
        .map(|&stone| count_stones(stone, 75, &mut memo))
        .sum();

    println!("Number of stones after 75 blinks: {}", total_stones);

    Ok(())
}

fn count_stones(
    stone: u64,
    remaining_blinks: usize,
    memo: &mut HashMap<(u64, usize), usize>,
) -> usize {
    if remaining_blinks == 0 {
        return 1; 
    }

    if let Some(&cached) = memo.get(&(stone, remaining_blinks)) {
        return cached;
    }

    let next_stones: Vec<u64> = match stone {
        0 => vec![1], 
        _ if stone.to_string().len() % 2 == 0 => {
            let digits = stone.to_string();
            let mid = digits.len() / 2;
            let left = digits[..mid].parse::<u64>().unwrap();
            let right = digits[mid..].parse::<u64>().unwrap();
            vec![left, right]
        }
        _ => {
            if let Some(multiplied) = stone.checked_mul(2024) {
                vec![multiplied]
            } else {
                vec![] 
            }
        }
    };

    let total_count: usize = next_stones
        .iter()
        .map(|&next_stone| count_stones(next_stone, remaining_blinks - 1, memo))
        .sum();

    memo.insert((stone, remaining_blinks), total_count);

    total_count
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

    // #[test]
    // fn test_solve_a_with_real_input() {
    //     let puzzle = get_puzzle('A');
    //     if let Some(input) = get_input_for_puzzle(&puzzle) {
    //         solve_a(&input);
    //         assert!(true, "Add your assertions here");
    //     } else {
    //         panic!("Input file not found for {:?}", puzzle);
    //     }
    // }

    // #[test]
    // #[ignore]
    // fn test_solve_b_with_real_input() {
    //     let puzzle = get_puzzle('B');
    //     if let Some(input) = get_input_for_puzzle(&puzzle) {
    //         solve_b(&input);
    //         assert!(true, "Add your assertions here");
    //     } else {
    //         panic!("Input file not found for {:?}", puzzle);
    //     }
    // }
}
