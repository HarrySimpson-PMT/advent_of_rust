use std::fs;
use std::path::Path;
mod aoc2024;
mod aoc2025;
mod comms;
mod solver;

#[allow(unused_imports)]
use comms::pico_sender::send_data_to_pico;

use std::time::Instant;

use crate::solver::DaySolver;

#[allow(unreachable_code)]
#[tokio::main]
async fn main() {
    let day_solver = aoc2025::day12::Day;
    let part = 1;
    let sample = false;
    let transmit_to_pico = false;
    if let Some(input_lines) =
        get_input_for_puzzle(day_solver.get_day(), day_solver.get_year(), sample)
    {
        let day = day_solver.get_day();
        println!();
        println!("-----------------------------------");
        let full_time_start = Instant::now();
        if part & 1 != 0 {
            let start_time = Instant::now();
            let _result = day_solver.solve_a(&input_lines).await;
            let duration = start_time.elapsed();
            println!("Time taken: {:.2?}", duration);
        }
        println!("-----------------------------------");
        if part & 2 != 0 {
            let start_time = Instant::now();
            let _result = day_solver.solve_b(&input_lines).await;
            let duration = start_time.elapsed();
            println!("Time taken: {:.2?}", duration);
        }
        let full_duration = full_time_start.elapsed();
        println!("-----------------------------------");
        println!("Total time taken for Day {}: {:.2?}", day, full_duration);
    } else {
        println!("Input file not found for puzzle {}", day_solver.get_day());
    }
    if transmit_to_pico {
        println!("-----------------------------------");
        let start_time = Instant::now();
        let somelines =
            match get_input_for_puzzle(day_solver.get_day(), day_solver.get_year(), sample) {
                Some(lines) => lines,
                None => {
                    println!("Input file not found for puzzle {}", day_solver.get_day());
                    return;
                }
            };
        let result = send_data_to_pico(&somelines).await;
        let duration = start_time.elapsed();
        match result {
            Ok(_) => println!("Pico recieved and returned results succesfully in {:.2?}", duration),
            Err(e) => println!("Error sending data to Pico: {:?}", e),
        }
    }
}

fn get_input_for_puzzle(day: u8, year: u16, sample: bool) -> Option<Vec<String>> {
    let daystring = if day < 10 {
        format!("day0{}", day)
    } else {
        format!("day{}", day)
    };
    let file_name = if sample { "sample.txt" } else { "file.txt" };
    let path_str = format!("primary_solver/inputs/{}/{}/{}", year, daystring, file_name);
    let input_path = Path::new(&path_str);
    // println!("{:?}", input_path);
    fs::read_to_string(input_path)
        .ok()
        .map(|content| content.lines().map(String::from).collect())
}
