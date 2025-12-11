use crate::solver::DaySolver;
use tokio::io;
use std::collections::{HashMap};

pub struct Day;

impl DaySolver for Day {
    async fn solve_a(&self, lines: &Vec<String>) -> io::Result<()> {
        solve_a(lines).await
    }

    async fn solve_b(&self, lines: &Vec<String>) -> io::Result<()> {
        solve_b(lines).await
    }

    fn get_day(&self) -> u8 {
        11
    }

    fn get_year(&self) -> u16 {
        2025
    }
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part A", Day.get_day());
    //input is like "xeq: mqb gzn"
    let mut connections: HashMap<String, (Vec<String>, u64)> = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let name = parts[0].to_string();
        let rest = parts[1];
        let rest_parts: Vec<&str> = rest.split(" ").collect();
        let mut conn: Vec<String> = Vec::new();
        for i in 0..rest_parts.len() {
            conn.push(rest_parts[i].to_string());
        }
        connections.insert(name, (conn, 0));
    }
    //start is conn "you" end is "out" find the number of ways for you to out
    fn find_paths(
        connections: &HashMap<String, (Vec<String>, u64)>,
        current: &String,
        end: &String,
        visited: &mut Vec<String>,
    ) -> u64 {
        if current == end {
            return 1;
        }
        let mut path_count = 0;
        if let Some((neighbors, _)) = connections.get(current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.push(neighbor.clone());
                    path_count += find_paths(connections, neighbor, end, visited);
                    visited.pop();
                }
            }
        }
        path_count
    }
    let mut visited: Vec<String> = Vec::new();
    visited.push("you".to_string());
    let result = find_paths(
        &connections,
        &"you".to_string(),
        &"out".to_string(),
        &mut visited,
    );

    println!("Result: {}", result);
    Ok(())
}

fn encode_location_flags(loc: &str, dac: bool, fft: bool) -> String {
    let mut result = loc.to_string();
    if dac && fft {
        result.push_str("11");
    } else if dac {
        result.push_str("10");
    } else if fft {
        result.push_str("01");
    } else {
        result.push_str("00");
    }
    result
}
fn decode_location_flags(loc: &str) -> (String, bool, bool) {
    let len = loc.len();
    let flags = &loc[len - 2..];
    let location = &loc[..len - 2];
    let dac = match flags {
        "10" | "11" => true,
        _ => false,
    };
    let fft = match flags {
        "01" | "11" => true,
        _ => false,
    };
    (location.to_string(), dac, fft)
}

// 9392593633280 low?
// 9392593633280
//331837854931968
pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part B", Day.get_day());
    let mut connections: HashMap<String, (Vec<String>, u64)> = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let name = parts[0].to_string();
        let rest = parts[1];
        let rest_parts: Vec<&str> = rest.split(" ").collect();
        let mut conn: Vec<String> = Vec::new();

        if name == "dac" {
            //from no path
            conn.clear();
            for i in 0..rest_parts.len() {
                conn.push(encode_location_flags(rest_parts[i], true, false));
            }
            let insert_name = encode_location_flags(&name, false, false);
            connections.insert(insert_name.clone(), (conn.clone(), 0));

            //from fft path
            conn.clear();
            for i in 0..rest_parts.len() {
                conn.push(encode_location_flags(rest_parts[i], true, true));
            }
            let insert_name = encode_location_flags(&name, false, true);
            connections.insert(insert_name.clone(), (conn.clone(), 0));
        } else if name == "fft" {
            //from no path
            conn.clear();
            for i in 0..rest_parts.len() {
                conn.push(encode_location_flags(rest_parts[i], false, true));
            }
            let insert_name = encode_location_flags(&name, false, false);
            connections.insert(insert_name.clone(), (conn.clone(), 0));

            //from dac path
            conn.clear();
            for i in 0..rest_parts.len() {
                conn.push(encode_location_flags(rest_parts[i], true, true));
            }
            let insert_name = encode_location_flags(&name, true, false);
            connections.insert(insert_name.clone(), (conn.clone(), 0));

        } else {
            conn.clear();
            for i in 0..rest_parts.len() {
                conn.push(encode_location_flags(rest_parts[i], false, false));
            }
            let insert_name = encode_location_flags(&name, false, false);
            connections.insert(insert_name, (conn.clone(), 0));

            conn.clear();
            for i in 0..rest_parts.len() {
                conn.push(encode_location_flags(rest_parts[i], true, false));
            }
            let insert_name_dac = encode_location_flags(&name, true, false);
            connections.insert(insert_name_dac, (conn.clone(), 0));

            conn.clear();
            for i in 0..rest_parts.len() {
                conn.push(encode_location_flags(rest_parts[i], false, true));
            }
            let insert_name_fft = encode_location_flags(&name, false, true);
            connections.insert(insert_name_fft, (conn.clone(), 0));

            conn.clear();
            for i in 0..rest_parts.len() {
                conn.push(encode_location_flags(rest_parts[i], true, true));
            }
            let insert_name_both = encode_location_flags(&name, true, true);
            connections.insert(insert_name_both, (conn.clone(), 0));
        }
    }

    let mut memo = HashMap::new();
    let result = dfs("svr00", &connections, &mut memo);

    println!("Result: {}", result);
    Ok(())
}

fn dfs(
    node: &str,
    connections: &HashMap<String, (Vec<String>, u64)>,
    memo: &mut HashMap<String, u128>,
) -> u128 {
    let key = node.to_string();
    if let Some(&val) = memo.get(&key) {
        return val;
    }
    if node.starts_with("out11") {
        return 1;
    }
    let mut total: u128 = 0;
    if let Some((neighbors, _)) = connections.get(&key) {
        for neigh in neighbors {
            total += dfs(&neigh, connections, memo);
        }
    }
    memo.insert(key, total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Puzzle;
    use crate::get_input_for_puzzle;
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
