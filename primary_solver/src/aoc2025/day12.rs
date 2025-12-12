use crate::solver::DaySolver;
use std::cell::Cell;
use std::collections::HashSet;
use std::usize;
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
        12
    }

    fn get_year(&self) -> u16 {
        2025
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Shape {
    id: u8,
    grid: [[bool; 3]; 3],
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Grid {
    width: u8,
    height: u8,
    required_shapes: [Cell<u8>; 6], // Cell allows interior mutability
}
impl Shape {
    fn load_from_str(lines: &[String], id: u8) -> Option<Self> {
        let mut grid = [[false; 3]; 3];
        for i in 0..3 {
            let line = &lines[i];
            let chars: Vec<char> = line.chars().collect();
            for j in 0..3 {
                grid[i][j] = chars[j] == '#';
            }
        }
        Some(Shape { id, grid })
    }
    fn get_orientation(&self, orientation: u8) -> [[bool; 3]; 3] {
        let mut new_grid = [[false; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                match orientation % 4 {
                    0 => new_grid[i][j] = self.grid[i][j],        
                    1 => new_grid[j][2 - i] = self.grid[i][j],     
                    2 => new_grid[2 - i][2 - j] = self.grid[i][j], 
                    3 => new_grid[2 - j][i] = self.grid[i][j],    
                    _ => (),
                }
            }
        }
        new_grid
    }
}
impl Grid {
    fn load_from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            return None;
        }
        let dims = parts[0];
        let required_str = parts[1].trim();

        let dim_parts: Vec<&str> = dims.split('x').collect();
        if dim_parts.len() != 2 {
            return None;
        }
        let width = dim_parts[0].parse::<u8>().ok()?;
        let height = dim_parts[1].parse::<u8>().ok()?;

        let req_parts: Vec<&str> = required_str.split(' ').collect();
        let required: [Cell<u8>; 6] = {
            let mut req_array: [Cell<u8>; 6] = Default::default();
            for i in 0..6 {
                req_array[i] =
                    Cell::new(req_parts.get(i).unwrap_or(&"0").trim().parse::<u8>().ok()?);
            }
            req_array
        };

        Some(Grid {
            width,
            height,
            required_shapes: required,
        })
    }
}

fn process_input(lines: &Vec<String>) -> (Vec<Shape>, Vec<Grid>) {
    let mut shapes = Vec::new();
    let mut grids = Vec::new();
    let sections: Vec<Vec<String>> = lines
        .split(|line| line.trim().is_empty())
        .filter(|section| !section.is_empty())
        .map(|section| section.to_vec())
        .collect();
    //all but the last secion are shapes
    for (i, section) in sections.iter().enumerate() {
        if i < sections.len() - 1 {
            //0,0
            let id = section[0].as_bytes()[0] - b'0';

            let shape_section = &section[1..4];
            if let Some(shape) = Shape::load_from_str(shape_section, id as u8) {
                shapes.push(shape);
            } else {
                println!("Failed to load shape with ID {}", id);
            }
        } else {
            for line in section {
                if let Some(grid) = Grid::load_from_line(line) {
                    grids.push(grid);
                } else {
                    println!("Failed to load grid from line: {}", line);
                }
            }
        }
    }
    (shapes, grids)
}

fn encode_state(placement_grid: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut state_key: Vec<u8> = Vec::new();
    for row in placement_grid {
        for &cell in row {
            state_key.push(cell);
        }
    }
    state_key
}
fn encode_shape_orientation(shape_id: u8, orientation: u8) -> u8 {
    (shape_id << 2) | (orientation & 0b11)
}
fn decode_shape_orientation(encoded: u8) -> (u8, u8) {
    let shape_id = encoded >> 2;
    let orientation = encoded & 0b11;
    (shape_id, orientation)
}
pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part A", Day.get_day());
    let mut result = 0;
    let (shapes, grids) = process_input(lines);
    for gridid in 0..grids.len() {
        println!("Processing grid {}", gridid);

        //first we prune impossible grids
        let dimensions = grids[gridid].width as usize * grids[gridid].height as usize;
        println!(
            "Grid dimensions: {}x{}={}",
            grids[gridid].width, grids[gridid].height, dimensions
        );
        let mut total_shape_area = 0;
        for shape_id in 0..grids[gridid].required_shapes.len() {
            let shape_count = grids[gridid].required_shapes[shape_id].get();
            let shape_area = shapes[shape_id]
                .grid
                .iter()
                .flatten()
                .filter(|&&b| b)
                .count();
            total_shape_area += shape_count as usize * shape_area;
        }
        println!("Total shape area: {}", total_shape_area);
        if total_shape_area > dimensions {
            println!("Skipping grid {} due to impossible area", gridid);
            continue;
        }

        let mut grid = grids[gridid].clone();
        let mut visited_states: HashSet<Vec<u8>> = HashSet::new();
        let mut validation_grid: Vec<Vec<bool>> =
            vec![vec![false; grid.width as usize]; grid.height as usize];
        let mut placement_grid: Vec<Vec<u8>> =
            vec![vec![0; grid.width as usize]; grid.height as usize];
        let mut passing: bool = false;

        for shape_id in 0..grid.required_shapes.len() {
            if grid.required_shapes[shape_id].get() == 0 {
                continue;
            }
            let shape = &shapes[shape_id];
            for orientation in 0..4 {
                let oriented_shape = shape.get_orientation(orientation);
                let start_x = 1;
                let start_y = 1;

                let encoded_shape = encode_shape_orientation(shape.id, orientation);

                placement_grid[start_y][start_x] = encoded_shape;
                let state_key = encode_state(&placement_grid);

                if !visited_states.contains(&state_key) {
                    visited_states.insert(state_key);
                    grid.required_shapes[shape.id as usize].set(
                        grid.required_shapes[shape.id as usize]
                            .get()
                            .saturating_sub(1),
                    );
                    for i in 0..3 {
                        for j in 0..3 {
                            if oriented_shape[i][j] {
                                let x = start_x + j - 1;
                                let y = start_y + i - 1;
                                validation_grid[y][x] = true;
                            }
                        }
                    }
                    //recurse

                    if dfs(
                        &mut visited_states,
                        &mut placement_grid,
                        &mut validation_grid,
                        &shapes,
                        &mut grid,
                        1,
                        2,
                    ) {
                        println!("Found valid placemetn for grid {}", gridid);
                        println!("Validation Grid:");
                        for row in &validation_grid {
                            let row_str: String =
                                row.iter().map(|&b| if b { '#' } else { '.' }).collect();
                            println!("{}", row_str);
                        }
                        result += 1;
                        passing = true;
                        break;
                    }
                    //backtrack
                    placement_grid[start_y][start_x] = 0;
                    grid.required_shapes[shape.id as usize].set(
                        grid.required_shapes[shape.id as usize]
                            .get()
                            .saturating_add(1),
                    );
                    for i in 0..3 {
                        for j in 0..3 {
                            if oriented_shape[i][j] {
                                let x = start_x + j - 1;
                                let y = start_y + i - 1;
                                validation_grid[y][x] = false;
                            }
                        }
                    }
                }
            }
            if passing {
                break;
            }
        }
    }

    println!("Result: {}", result);
    Ok(())
}

fn dfs(
    visited: &mut HashSet<Vec<u8>>,
    placement_grid: &mut Vec<Vec<u8>>,
    validation_grid: &mut Vec<Vec<bool>>,
    shapes: &Vec<Shape>,
    grid: &mut Grid,
    mut x: usize,
    mut y: usize
) -> bool {
    let mut all_placed = true;
    if y >= grid.height as usize - 1 {
        y = 1;
        x += 1;
    }
    if x >= grid.width as usize - 1 {
        //sum the required shapes to see if all placed
        let sum = grid
            .required_shapes
            .iter()
            .map(|c| c.get() as usize)
            .sum::<usize>();
        return sum == 0;
    }

    for i in 0..grid.required_shapes.len() {
        if grid.required_shapes[i].get() == 0 {
            continue;
        }

        all_placed = false;
        let shape_id = i as u8;
        let shape = shapes.iter().find(|s| s.id == shape_id).unwrap();
        for orientation in 0..4 {
            let oriented_shape = shape.get_orientation(orientation);

            let mut can_place = true;
            for i in 0..3 {
                for j in 0..3 {
                    let gx = x as usize + j - 1;
                    let gy = y as usize + i - 1;
                    let cell_filled = oriented_shape[i][j];
                    if cell_filled {
                        if gx >= grid.width as usize
                            || gy >= grid.height as usize
                            || validation_grid[gy][gx]
                        {
                            can_place = false;
                        }
                    }
                }
            }
            if can_place {
                let encoded_shape = encode_shape_orientation(shape.id, orientation);
                placement_grid[y][x] = encoded_shape;
                let state_key = encode_state(&placement_grid);
                if !visited.contains(&state_key) {
                    visited.insert(state_key);
                    for i in 0..3 {
                        for j in 0..3 {
                            if oriented_shape[i][j] {
                                let gx = x as usize + j - 1;
                                let gy = y as usize + i - 1;
                                validation_grid[gy][gx] = true;
                            }
                        }
                    }
                    grid.required_shapes[shape.id as usize].set(
                        grid.required_shapes[shape.id as usize]
                            .get()
                            .saturating_sub(1),
                    );

                    //recurse
                    if dfs(
                        visited,
                        placement_grid,
                        validation_grid,
                        shapes,
                        grid,
                        x,
                        y + 1,
                    ) {
                        return true;
                    }
                    //backtrack
                    placement_grid[y][x] = 0;
                    grid.required_shapes[shape.id as usize].set(
                        grid.required_shapes[shape.id as usize]
                            .get()
                            .saturating_add(1),
                    );
                    for i in 0..3 {
                        for j in 0..3 {
                            if oriented_shape[i][j] {
                                let gx = x as usize + j - 1;
                                let gy = y as usize + i - 1;
                                validation_grid[gy][gx] = false;
                                placement_grid[gy][gx] = 0;
                            }
                        }
                    }
                }
            }
        }
    }
    if all_placed {
        return true;
    }
    return dfs(
        visited,
        placement_grid,
        validation_grid,
        shapes,
        grid,
        x,
        y + 1,
    );
}

pub async fn solve_b(_lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part B", Day.get_day());
    let result = 0;

    println!("Result: {}", result);
    Ok(())
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
