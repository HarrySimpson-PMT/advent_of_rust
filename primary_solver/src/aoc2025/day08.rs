use crate::solver::DaySolver;
use tokio::io;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub struct Day;

impl DaySolver for Day {
    async fn solve_a(&self, lines: &Vec<String>) -> io::Result<()> {
        solve_a(lines).await
    }

    async fn solve_b(&self, lines: &Vec<String>) -> io::Result<()> {
        solve_b(lines).await
    }

    fn get_day(&self) -> u8 {
        8
    }

    fn get_year(&self) -> u16 {
        2025
    }
}

//3d space structure
struct Space3D {
    id: u32,
    x: i64,
    y: i64,
    z: i64,
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part A", Day.get_day());
    let result = 0;

    let mut points: Vec<Space3D> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let coords: Vec<i64> = line
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();
        if coords.len() != 3 {
            continue;
        }
        points.push(Space3D {
            id: i as u32,
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }

    let mut heap: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dx = points[i].x - points[j].x;
            let dy = points[i].y - points[j].y;
            let dz = points[i].z - points[j].z;
            println!("Point {} to Point {}: dx={}, dy={}, dz={}", i, j, dx, dy, dz);
            let dist = (dx * dx + dy * dy + dz * dz) as i64;
            heap.push((-dist, i, j));
        }
    }

    let mut connections: HashMap<usize, Vec<usize>> = HashMap::new();
    let target_connections = 1000;
    let mut current_connections = 0;
    while let Some((_neg_dist, i, j)) = heap.pop() {
        if connections.get(&i).map_or(0, |v| v.len()) >=
            target_connections ||
            connections.get(&j).map_or(0, |v| v.len()) >= target_connections
        {
            continue;
        }
        connections.entry(i).or_insert(Vec::new()).push(j);
        connections.entry(j).or_insert(Vec::new()).push(i);
        if connections.len() >= points.len() {
            break;
        }
        current_connections += 1;
        if current_connections >= target_connections {
            break;
        }
    }
    println!("Connections established: {}", connections.len());

    let mut visited: Vec<bool> = vec![false; points.len()];
    let mut group_sizes: Vec<usize> = Vec::new();
    for i in 0..points.len() {
        if visited[i] {
            continue;
        }
        let mut stack: Vec<usize> = vec![i];
        let mut group_size = 0;
        while let Some(node) = stack.pop() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            group_size += 1;
            if let Some(neighbors) = connections.get(&node) {
                for &neighbor in neighbors {
                    if !visited[neighbor] {
                        stack.push(neighbor);
                    }
                }
            }
        }
        group_sizes.push(group_size);
    }
    group_sizes.sort_unstable_by(|a, b| b.cmp(a));
    let top3_product: usize = group_sizes.iter().take(3).product();
    println!("Top 3 group sizes product: {}", top3_product);
   

    println!("Result: {}", result);
    Ok(())
}


pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part B", Day.get_day());
    let result = 0;
    let mut points: Vec<Space3D> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let coords: Vec<i64> = line
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();
        if coords.len() != 3 {
            continue;
        }
        points.push(Space3D {
            id: i as u32,
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }
    let mut heap: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dx = points[i].x - points[j].x;
            let dy = points[i].y - points[j].y;
            let dz = points[i].z - points[j].z;
            let dist = (dx * dx + dy * dy + dz * dz) as i64;
            heap.push((-dist, i, j));
        }
    }
    let mut parent: Vec<usize> = (0..points.len()).collect();
    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }
    fn union(parent: &mut Vec<usize>, x: usize, y: usize)
    {
        let root_x = find(parent, x);
        let root_y = find(parent, y);
        if root_x != root_y {
            parent[root_y] = root_x;
        }
    }
    while let Some((neg_dist, i, j)) = heap.pop() {
        if find(&mut parent, i) == find(&mut parent, j) {
            continue;
        }
        union(&mut parent, i, j);
        let mut unique_parents: HashMap<usize, usize> = HashMap::new();
        for k in 0..points.len() {
            let root = find(&mut parent, k);
            *unique_parents.entry(root).or_insert(0) += 1;
        }
        if unique_parents.len() <= 1 {
            let product = points[i].x * points[j].x;
            println!("Last connection between point {} and point {} with distance {}. Product of x coords: {}", i, j, -neg_dist, product);
            break;
        }       
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
