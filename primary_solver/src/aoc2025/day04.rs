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
        4
    }

    fn get_year(&self) -> u16 {
        2025
    }
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day 4, Part A");
    let mut result = 0;
    for i in 0..lines.len() {
        let line = &lines[i];
        for j in 0..line.len() {
            let c = line.chars().nth(j).unwrap();
            if c == '@' {
                let mut count = 0;
                for di in -1..2 {
                    for dj in -1..2 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni < 0
                            || ni >= lines.len() as isize
                            || nj < 0
                            || nj >= line.len() as isize
                        {
                            continue;
                        }
                        let nc = lines[ni as usize].chars().nth(nj as usize).unwrap();
                        if nc == '@' {
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    result += 1;
                } else {
                }
            } 
        }
    }
    println!("Result is {}", result);
    Ok(())
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    let mut lines = lines.clone();
    println!("Solving Day 4, Part B");
    //we need to create an adjacency list of the graph
    let mut result = 0;
    //now we need a queue to handle all items with less than 4 connections and process them
    let mut queue: Vec<(usize, usize)> = vec![];
    //2d vec of vec(x, y) of size lines.len() x lines[0].len()
    let mut items: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![vec![]; lines[0].len()]; lines.len()];
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let c = lines[i].chars().nth(j).unwrap();
            if c == '@' {
                //we scan
                for di in -1..2 {
                    for dj in -1..2 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni < 0
                            || ni >= lines.len() as isize
                            || nj < 0
                            || nj >= lines[i].len() as isize
                        {
                            continue;
                        }
                        let nc = lines[ni as usize].chars().nth(nj as usize).unwrap();
                        if nc == '@' {
                            items[i][j].push((ni as usize, nj as usize));
                        }
                    }
                }
                if items[i][j].len() < 4 {
                    queue.push((i, j));
                }
            }
        }
    }
    while !queue.is_empty() {
        let (i, j) = queue.pop().unwrap();
        let connections = items[i][j].len();
        if connections >= 4 {
            continue;
        }
        //check if @ has been removed from lines, if so continue else remove it and process
        let c = lines[i].chars().nth(j).unwrap();
        if c != '@' {
            continue;
        }
        
        result += 1;
        lines[i].replace_range(j..=j, ".");

        //remove this item from all its neighbors
        let neighbors = items[i][j].clone();
        for (ni, nj) in neighbors {
            let index = items[ni][nj]
                .iter()
                .position(|&(x, y)| x == i && y == j)
                .unwrap();
            items[ni][nj].remove(index);
            if items[ni][nj].len() < 4 {
                queue.push((ni, nj));
            }
        }
        items[i][j].clear();
    }
    println!("Result is {}", result);
    //print lines
    // for line in lines {
    //     println!("{}", line);
    // }

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
