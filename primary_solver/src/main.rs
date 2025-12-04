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
    let day = 1;
    let year = 2025;
    let part = 3;
    let sample = false;

    if let Some(input_lines) = get_input_for_puzzle(day, year, sample) {
        println!();
        println!("-----------------------------------");
        let full_time_start = Instant::now();
        if part & 1 != 0 {
            let start_time = Instant::now();
            let _result = aoc2025::day01::solve_a(&input_lines).await;
            let duration = start_time.elapsed();
            println!("Time taken: {:.2?}", duration);
        }
        println!("-----------------------------------");
        if part & 2 != 0 {
            let start_time = Instant::now();
            let _result = aoc2025::day01::solve_b(&input_lines).await;
            let duration = start_time.elapsed();
            println!("Time taken: {:.2?}", duration);
        }    
        let full_duration = full_time_start.elapsed();
        println!("-----------------------------------");
        println!("Total time taken for Day {}: {:.2?}", day, full_duration);
        
    } else {
        println!("Input file not found for puzzle {}", day);
    }

    return; //this is used to stop execution before sending to pico

    let somelines = match get_input_for_puzzle(day, year, sample) {
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

fn get_input_for_puzzle(day: i32, year: i32, sample: bool
) -> Option<Vec<String>> {
    let daystring = if day < 10 {
        format!("day0{}", day)
    } else {
        format!("day{}", day)
    };
    let file_name = if sample {
        "sample.txt"
    } else {
        "file.txt"
    };
    let path_str = format!(
        "primary_solver/inputs/{}/{}/{}",
        year,
        daystring,
        file_name
    );
    let input_path = Path::new(&path_str);
    print!("{:?}", input_path);
    fs::read_to_string(input_path)
        .ok()
        .map(|content| content.lines().map(String::from).collect())
}
