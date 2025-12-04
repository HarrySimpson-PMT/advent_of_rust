use std::fs;
use std::path::Path;
mod aoc2024;
mod aoc2025;
mod comms;

#[allow(unused_imports)]
use comms::pico_sender::send_data_to_pico;

use std::time::Instant;

#[allow(unreachable_code)]
#[tokio::main]
async fn main() {
    let day = 4;
    let year = 2025;

    if let Some(input_lines) = get_input_for_puzzle(day, year) {
        print!("Input lines loaded: {}\n", input_lines.len());
        let start_time = Instant::now();
        let result = aoc2025::day04::solve_b(&input_lines).await;
        let duration = start_time.elapsed();

        println!("Result: {:?}", result);
        println!("Time taken: {:.2?}", duration);
    } else {
        println!("Input file not found for puzzle {}", day);
    }

    return;

    // This will send to whatever is running on the Pico side
    let somelines = match get_input_for_puzzle(day, year) {
        Some(lines) => lines,
        None => {
            println!("Input file not found for puzzle {}", day);
            return;
        }
    };
    let result = send_data_to_pico(&somelines).await;
    match result {
        Ok(_) => println!("Data sent to Pico successfully"),
        Err(e) => println!("Error sending data to Pico: {:?}", e),
    }

    

}

fn get_input_for_puzzle(day: i32, year: i32) -> Option<Vec<String>> {
    let daystring = if day < 10 {
        format!("day0{}", day)
    } else {
        format!("day{}", day)
    };
    let path_str = format!("primary_solver/inputs/{:02}/{:02}/file.txt", year, daystring);
    let input_path = Path::new(&path_str);
    print!("{:?}", input_path);
    fs::read_to_string(input_path)
        .ok()
        .map(|content| content.lines().map(String::from).collect())
}
