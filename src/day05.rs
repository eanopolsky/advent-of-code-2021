use std::cmp;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct VentLine {
    start_point: Point,
    end_point: Point,
}

impl VentLine {
    fn is_horizontal_or_vertical(&self) -> bool {
        if self.is_horizontal() || self.is_vertical() {
            return true;
        }
        false
    }

    fn is_horizontal(&self) -> bool {
        if self.start_point.y == self.end_point.y {
            return true;
        }
        false
    }

    fn is_vertical(&self) -> bool {
        if self.start_point.x == self.end_point.x {
            return true;
        }
        false
    }

    fn get_points_on_line(&self) -> Vec<Point> {
        let mut points_on_line = Vec::new();
        if self.is_horizontal() {
            let x_range = match self.start_point.x < self.end_point.x {
                true => (self.start_point.x)..=(self.end_point.x),
                false => (self.end_point.x)..=(self.start_point.x),
            };
            for x in x_range {
                points_on_line.push(Point {
                    x: x,
                    y: self.start_point.y,
                });
            }
        } else if self.is_vertical() {
            let y_range = match self.start_point.y < self.end_point.y {
                true => (self.start_point.y)..=(self.end_point.y),
                false => (self.end_point.y)..=(self.start_point.y),
            };
            for y in y_range {
                points_on_line.push(Point {
                    x: self.start_point.x,
                    y: y,
                });
            }
        } else {
            let mut x = self.start_point.x;
            let mut y = self.start_point.y;
            let x_step = match self.start_point.x < self.end_point.x {
                true => 1,
                false => -1,
            };
            let y_step = match self.start_point.y < self.end_point.y {
                true => 1,
                false => -1,
            };
            while x != self.end_point.x {
                points_on_line.push(Point { x, y });
                x += x_step;
                y += y_step;
            }
            points_on_line.push(Point { x, y });
        }
        points_on_line
    }
}

fn get_vent_lines(puzzle_input: String) -> Vec<VentLine> {
    puzzle_input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line_definition| -> VentLine {
            // println!("Processing input line: {}", line_definition);
            let mut vent_points = line_definition
                .split(" -> ")
                .map(|point_definition| -> Point {
                    let mut ordinate_iter = point_definition
                        .split(",")
                        .map(|n| -> i32 { (n.parse::<i32>()).unwrap() });

                    Point {
                        x: ordinate_iter.next().unwrap(),
                        y: ordinate_iter.next().unwrap(),
                    }
                });

            VentLine {
                start_point: vent_points.next().unwrap(),
                end_point: vent_points.next().unwrap(),
            }
        })
        .collect::<Vec<VentLine>>()
}

fn make_ocean_floor(vent_lines: &Vec<&VentLine>) -> Vec<Vec<u32>> {
    let max_x: usize = vent_lines
        .iter()
        .map(|vent_line| cmp::max(vent_line.start_point.x, vent_line.end_point.x))
        .max()
        .unwrap() as usize;
    let max_y: usize = vent_lines
        .iter()
        .map(|vent_line| cmp::max(vent_line.start_point.y, vent_line.end_point.y))
        .max()
        .unwrap() as usize;

    // println!("Got max_x: {}, max_y: {}", max_x, max_y);
    vec![vec![0; max_x + 1]; max_y + 1]
}

pub(crate) fn solve_part_1(puzzle_input: String) -> u32 {
    let vent_lines = get_vent_lines(puzzle_input);

    let horizontal_and_vertical_vent_lines = vent_lines
        .iter()
        .filter(|vent_line| vent_line.is_horizontal_or_vertical())
        .collect::<Vec<&VentLine>>();

    let mut ocean_floor = make_ocean_floor(&horizontal_and_vertical_vent_lines);

    for vent_lines in &horizontal_and_vertical_vent_lines {
        for vent_line_point in vent_lines.get_points_on_line() {
            ocean_floor[vent_line_point.y as usize][vent_line_point.x as usize] += 1;
        }
    }
    let mut overlap_count: u32 = 0;
    for ocean_floor_row in ocean_floor {
        for vent_count in ocean_floor_row {
            if vent_count > 1 {
                overlap_count += 1;
            }
        }
    }

    overlap_count
}

// fn render_ocean_floor(ocean_floor: &Vec<Vec<u32>>) {
//     for ocean_floor_row in ocean_floor {
//         let rendered_row = ocean_floor_row
//             .iter()
//             .map(|vent_count| vent_count.to_string())
//             .map(|vent_count_string| {
//                 if vent_count_string == "0" {
//                     return ".".to_string();
//                 }
//                 vent_count_string
//             })
//             .collect::<Vec<String>>()
//             .join("");
//         println!("{}", rendered_row);
//     }
// }

pub(crate) fn solve_part_2(puzzle_input: String) -> u32 {
    let vent_lines = get_vent_lines(puzzle_input);
    let vent_lines = vent_lines.iter().collect::<Vec<&VentLine>>();

    let mut ocean_floor = make_ocean_floor(&vent_lines);

    for vent_line in &vent_lines {
        for vent_line_point in vent_line.get_points_on_line() {
            ocean_floor[vent_line_point.y as usize][vent_line_point.x as usize] += 1;
        }
        // println!("Applied vent line {:?}", vent_line);
        // render_ocean_floor(&ocean_floor);
    }
    let mut overlap_count: u32 = 0;
    for ocean_floor_row in &ocean_floor {
        for vent_count in ocean_floor_row {
            if vent_count > &1 {
                overlap_count += 1;
            }
        }
    }
    // render_ocean_floor(&ocean_floor);
    overlap_count
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(5)), 5442);
}

#[test]
fn test_part_2() {
    assert_eq!(solve_part_2(test_helpers::load_puzzle_input(5)), 19571);
}
