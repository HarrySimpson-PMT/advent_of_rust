use crate::solver::DaySolver;
use tokio::io;

use image::{Rgb, RgbImage};
use imageproc::drawing::draw_line_segment_mut;

pub struct Day;

impl DaySolver for Day {
    async fn solve_a(&self, lines: &Vec<String>) -> io::Result<()> {
        solve_a(lines).await
    }

    async fn solve_b(&self, lines: &Vec<String>) -> io::Result<()> {
        solve_b(lines).await
    }

    fn get_day(&self) -> u8 {
        9
    }

    fn get_year(&self) -> u16 {
        2025
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
    len_sq: i64, // squared length — exact, sortable, no overflow
}

pub async fn solve_a(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part A", Day.get_day());
    let mut result = 0;
    //input is x,y
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            let pointa: Point = {
                let parts: Vec<&str> = lines[i].split(',').collect();
                Point {
                    x: parts[0].parse().unwrap(),
                    y: parts[1].parse().unwrap(),
                }
            };
            let pointb: Point = {
                let parts: Vec<&str> = lines[j].split(',').collect();
                Point {
                    x: parts[0].parse().unwrap(),
                    y: parts[1].parse().unwrap(),
                }
            };
            // let area: i64 = ((pointb.x - pointa.x).abs() +1) * ((pointb.y - pointa.y).abs() +1) as i64;
            let area: i64 =
                ((pointb.x - pointa.x).abs() as i64 + 1) * ((pointb.y - pointa.y).abs() as i64 + 1);
            if result < area {
                result = area;
            }
        }
    }

    println!("Result: {}", result);
    Ok(())
}

pub fn draw_grid(grid: &Vec<Vec<char>>) {
    //clear screen
    print!("\x1B[2J\x1B[1;1H");
    for row in grid {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}

pub async fn solve_b(lines: &Vec<String>) -> io::Result<()> {
    println!("Solving Day {}, Part B", Day.get_day());
    let mut lines_vec: Vec<Line> = Vec::new();
    let mut points_vec: Vec<Point> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        points_vec.push(Point {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
        });
    }

    // create all teh lines
    for i in 0..lines.len() - 1 {
        let start_point: Point = {
            let parts: Vec<&str> = lines[i].split(',').collect();
            Point {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
            }
        };
        let end_point: Point = {
            let parts: Vec<&str> = lines[i + 1].split(',').collect();
            Point {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
            }
        };
        let dx = end_point.x as i64 - start_point.x as i64;
        let dy = end_point.y as i64 - start_point.y as i64;
        let len_sq = dx * dx + dy * dy; 

        lines_vec.push(Line {
            start: start_point,
            end: end_point,
            len_sq, 
        });
    }
    let first_point: Point = {
        let parts: Vec<&str> = lines[0].split(',').collect();
        Point {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
        }
    };
    let last_point: Point = {
        let parts: Vec<&str> = lines[lines.len() - 1].split(',').collect();
        Point {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
        }
    };
    let dx = last_point.x as i64 - first_point.x as i64;
    let dy = last_point.y as i64 - first_point.y as i64;
    let len_sq = dx * dx + dy * dy; 
    lines_vec.push(Line {
        start: last_point,
        end: first_point,
        len_sq, 
    });
    // let (min_x, max_x, min_y, max_y) = build_boundary(&lines_vec);

    lines_vec.sort_by(|a, b| b.len_sq.cmp(&a.len_sq));
    let big_north_line = &lines_vec[1];
    let north_right_point = if big_north_line.start.x > big_north_line.end.x {
        big_north_line.start
    } else {
        big_north_line.end
    };
    let big_south_line = &lines_vec[0];
    let south_right_point = if big_south_line.start.x > big_south_line.end.x {
        big_south_line.start
    } else {
        big_south_line.end
    };

    
    let points_north_west = points_vec
        .iter()
        .filter(|p| p.x < north_right_point.x)
        .filter(|p| p.y > north_right_point.y)
        
        .collect::<Vec<&Point>>();
    let points_south_west = points_vec
        .iter()
        .filter(|p| p.x < south_right_point.x)
        .filter(|p| p.y < south_right_point.y)
        .collect::<Vec<&Point>>();


    // let data_w = max_x - min_x + 1;
    // let data_h = max_y - min_y + 1;

    // let scale_x = 990.0 / data_w as f32;
    // let scale_y = 990.0 / data_h as f32;
    // let scale = scale_x.min(scale_y);

    // let offset_x = (1000.0 - data_w as f32 * scale) / 2.0;
    // let offset_y = (1000.0 - data_h as f32 * scale) / 2.0;

    // let mut img = RgbImage::new(1000, 1000);
    // for p in img.pixels_mut() {
    //     *p = Rgb([30, 30, 40]);
    // }
    // for line in &lines_vec {
    //     draw_line(&mut img, line, min_x, min_y, scale, offset_x, offset_y);
    // }
    // draw_circle(
    //     &mut img,
    //     north_right_point,
    //     10,
    //     min_x,
    //     min_y,
    //     scale,
    //     offset_x,
    //     offset_y,
    //     Rgb([255, 0, 0]),
    // );
    // draw_circle(
    //     &mut img,
    //     south_right_point,
    //     10,
    //     min_x,
    //     min_y,
    //     scale,
    //     offset_x,
    //     offset_y,
    //     Rgb([0, 255, 255]),
    // );
    let mut largest_area: u64 = 0;
    let mut largest_point: Option<Point> = None;
    for p in &points_north_west {
        let mut is_valid = true;
        //check if valid ie no other points inside the rectangle
        for op in &points_north_west {
            if **op == **p {
                continue;
            }
            if op.x >= p.x && op.x <= north_right_point.x && op.y <= p.y && op.y >= north_right_point.y
            {
                is_valid = false;
                break;
            }
        }
        if !is_valid {
            continue;
        }

        let current_area: u64 = ((north_right_point.x - p.x).abs() as u64 + 1)
            * ((north_right_point.y - p.y).abs() as u64 + 1);
        if current_area > largest_area {
            largest_area = current_area;
            largest_point = Some(**p);
        }
    }
    //draw larger north west rectangle
    if let Some(_p) = largest_point {
        // draw_rectangle(
        //     &mut img,
        //     p,
        //     north_right_point,
        //     min_x,
        //     min_y,
        //     scale,
        //     offset_x,
        //     offset_y,
        //     Rgb([255, 100, 100]),
        // );
        // println!(
        //     "Largest North-West rectangle corner at ({}, {}) with area {}",
        //     p.x, p.y, largest_area
        // );
    }
    let mut result = largest_area;

    //now we check the south west points
    let mut largest_area: u64 = 0;
    let mut largest_point: Option<Point> = None;
    for p in &points_south_west {
        let mut is_valid = true;
        //check if valid ie no other points inside the rectangle
        for op in &points_south_west {
            if **op == **p {
                continue;
            }
            if op.x >= p.x && op.x <= south_right_point.x && op.y >= p.y && op.y <= south_right_point.y
            {
                is_valid = false;
                break;
            }
        }
        if !is_valid {
            continue;
        }
        let current_area: u64 = ((south_right_point.x - p.x).abs() as u64 + 1)
            * ((south_right_point.y - p.y).abs() as u64 + 1);
        if current_area > largest_area {
            largest_area = current_area;
            largest_point = Some(**p);
        }
    }
    //draw larger south west rectangle
    if let Some(_p) = largest_point {
        // draw_rectangle(
        //     &mut img,
        //     p,
        //     south_right_point,
        //     min_x,
        //     min_y,
        //     scale,
        //     offset_x,
        //     offset_y,
        //     Rgb([100, 255, 255]),
        // );
        // println!(
        //     "Largest South-West rectangle corner at ({}, {}) with area {}",
        //     p.x, p.y, largest_area
        // );
    }
    if largest_area > result {
        result = largest_area;
    }
    
    // img.save("day09_partb_output.png").unwrap();

    println!("Result: {}", result);
    Ok(())
}

fn build_boundary(lines: &Vec<Line>) -> (i32, i32, i32, i32) {
    let (min_x, max_x, min_y, max_y) = lines.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(mx,m_x, my, m_y), line| {
            let x1 = line.start.x.min(line.end.x);
            let x2 = line.start.x.max(line.end.x);
            let y1 = line.start.y.min(line.end.y);
            let y2 = line.start.y.max(line.end.y);
            (
                mx.min(x1).min(x2),
                m_x.max(x1).max(x2),
                my.min(y1).min(y2),
                m_y.max(y1).max(y2),
            )
        },
    );
    (min_x, max_x, min_y, max_y)
}

fn draw_line(
    img: &mut RgbImage,
    line: &Line,
    min_x: i32,
    min_y: i32,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
) {
    let to_img = |p: Point| -> (f32, f32) {
        let sx = ((p.x - min_x) as f32) * scale + offset_x;
        let sy = ((p.y - min_y) as f32) * scale + offset_y;
        (sx, 999.999 - sy)  // keep as f32, no early truncate
    };
    let (x1, y1) = to_img(line.start);
    let (x2, y2) = to_img(line.end);

    draw_line_segment_mut(img, (x1, y1), (x2, y2), Rgb([0, 255, 120]));
}

fn draw_circle(
    img: &mut RgbImage,
    point: Point,
    radius: i32,
    min_x: i32,
    min_y: i32,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
    color: Rgb<u8>,
) {
    let sx = ((point.x - min_x) as f32) * scale + offset_x;
    let sy = ((point.y - min_y) as f32) * scale + offset_y;
    let ix = sx.round() as i32;
    let iy = (999.999 - sy).round() as i32;
    let r = (radius as f32 * scale).max(2.0) as i32;

    let _ = imageproc::drawing::draw_filled_circle_mut(img, (ix, iy), r, color);
    // ignore return; it fails only on absurd radius
}

fn draw_rectangle(
    img: &mut RgbImage,
    p1: Point,
    p2: Point,
    min_x: i32,
    min_y: i32,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
    color: Rgb<u8>,
) {
    let to_img = |p: Point| {
        let sx = ((p.x - min_x) as f32) * scale + offset_x;
        let sy = ((p.y - min_y) as f32) * scale + offset_y;
        (sx.round() as i32, (999.999 - sy).round() as i32)
    };

    let (x1, y1) = to_img(p1);
    let (x2, y2) = to_img(p2);

    let left   = x1.min(x2);
    let right  = x1.max(x2);
    let top    = y1.min(y2);  // smallest Y in image space = top of screen
    let bottom = y1.max(y2);

    if right > left && bottom > top {
        let rect = imageproc::rect::Rect::at(left, top)
            .of_size((right - left) as u32, (bottom - top) as u32);
        imageproc::drawing::draw_filled_rect_mut(img, rect, color);
    }
}

fn draw_lines(
    lines: &Vec<Line>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    filename: &str,
) -> RgbImage {
    let data_w = max_x - min_x + 1;
    let data_h = max_y - min_y + 1;

    let scale_x = 990.0 / data_w as f32;
    let scale_y = 990.0 / data_h as f32;
    let scale = scale_x.min(scale_y);

    let offset_x = (1000.0 - data_w as f32 * scale) / 2.0;
    let offset_y = (1000.0 - data_h as f32 * scale) / 2.0;

    let mut img = RgbImage::new(1000, 1000);
    for p in img.pixels_mut() {
        *p = Rgb([30, 30, 40]);
    }

    for line in lines {
        let x1 = (line.start.x - min_x) as f32 * scale + offset_x;
        let y1 = (line.start.y - min_y) as f32 * scale + offset_y;
        let x2 = (line.end.x - min_x) as f32 * scale + offset_x;
        let y2 = (line.end.y - min_y) as f32 * scale + offset_y;

        let iy1 = 999.999 - y1;
        let iy2 = 999.999 - y2;

        draw_line_segment_mut(&mut img, (x1, iy1), (x2, iy2), Rgb([0, 255, 120]));
    }
    let north_line = &lines[0];
    let south_line = &lines[1];

    //right-most point for each line
    let north_point = if north_line.start.x > north_line.end.x {
        north_line.start
    } else {
        north_line.end
    };
    let south_point = if south_line.start.x > south_line.end.x {
        south_line.start
    } else {
        south_line.end
    };
    let to_img = |p: Point| -> (i32, i32) {
        let sx = ((p.x - min_x) as f32) * scale + offset_x;
        let sy = ((p.y - min_y) as f32) * scale + offset_y;
        (sx as i32, 999 - sy as i32) // flip Y and clamp to 0..999
    };

    let (nx, ny) = to_img(north_point);
    let (sx, sy) = to_img(south_point);

    let radius = (10.0 * scale).max(3.0) as i32; // never tiny

    if nx >= -radius && nx < 1000 + radius && ny >= -radius && ny < 1000 + radius {
        imageproc::drawing::draw_filled_circle_mut(&mut img, (nx, ny), radius, Rgb([255, 0, 0]));
    }
    if sx >= -radius && sx < 1000 + radius && sy >= -radius && sy < 1000 + radius {
        imageproc::drawing::draw_filled_circle_mut(&mut img, (sx, sy), radius, Rgb([0, 255, 255]));
    }

    img.save(filename).unwrap();
    img
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
