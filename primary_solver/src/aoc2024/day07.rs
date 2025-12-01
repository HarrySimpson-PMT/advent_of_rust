use tokio::io;

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 7, Part A");

    fn evaluate_combinations(
        numbers: &Vec<i64>,
        index: usize,
        current_value: i64,
        target: i64,
    ) -> bool {
        if index == numbers.len() {
            return current_value == target;
        }

        let next_value = numbers[index];
        evaluate_combinations(numbers, index + 1, current_value + next_value, target)
            || evaluate_combinations(numbers, index + 1, current_value * next_value, target)
    }

    let mut total_sum = 0;
    for line in lines {
        if let Some((target_str, numbers_str)) = line.split_once(":") {
            let target: i64 = target_str.trim().parse().unwrap_or(0);
            let numbers: Vec<i64> = numbers_str
                .split_whitespace()
                .map(|n| n.parse().unwrap_or(0))
                .collect();

            if evaluate_combinations(&numbers, 1, numbers[0], target) {
                total_sum += target;
            }
        }
    }

    println!("Total sum of valid targets: {}", total_sum);
    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 7, Part B");

    fn digit_count(mut n: i64) -> i64 {
        if n == 0 {
            return 1;
        }
        let mut count = 0;
        while n > 0 {
            n /= 10;
            count += 1;
        }
        count
    }

    fn concat(a: i64, b: i64) -> i64 {
        let digits = digit_count(b);
        a * 10_i64.pow(digits as u32) + b
    }
    
    fn evaluate_combinations(
        numbers: &[i64],
        mut index: usize,
        acc: i64,
        target: i64,
    ) -> bool {
        if index == numbers.len() {
            return acc == target
        }
        let current_number = numbers[index];
        if acc>target {
            return false
        }
        index += 1;

        if evaluate_combinations(numbers, index , acc + current_number, target) {
            return true;
        }

        if evaluate_combinations(numbers, index , acc * current_number, target) {
            return true;
        }

        let merged = concat(acc, current_number);
        if evaluate_combinations(numbers, index, merged, target) {
            return true;
        }

        false
    }

    let mut total_sum = 0;
    for line in lines {
        if let Some((target_str, numbers_str)) = line.split_once(':') {
            let target: i64 = target_str.trim().parse().unwrap_or(0);
            let numbers: Vec<i64> = numbers_str
                .split_whitespace()
                .map(|n| n.parse().unwrap_or(0))
                .collect();

            if numbers.is_empty() {
                continue;
            }

            if numbers.len() == 1 {
                if numbers[0] == target {
                    total_sum += target;
                }
            } else {
                if evaluate_combinations(&numbers, 1, numbers[0], target) {
                    total_sum += target;
                }
            }
        }
    }

    println!("Total sum of valid targets: {}", total_sum);
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
