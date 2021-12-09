struct HeightMap {
    // map_data[y][x]
    map_data: Vec<Vec<u8>>,
}

impl HeightMap {
    fn get_max_x(&self) -> usize {
        self.map_data[0].len() - 1
    }
    fn get_max_y(&self) -> usize {
        self.map_data.len() - 1
    }
    fn get_neighbor_heights(&self, x: usize, y: usize) -> Vec<u8> {
        let mut neighbor_heights: Vec<u8> = Vec::new();
        if x != 0 {
            neighbor_heights.push(self.map_data[y][x - 1]);
        }
        if x != self.get_max_x() {
            neighbor_heights.push(self.map_data[y][x + 1]);
        }
        if y != 0 {
            neighbor_heights.push(self.map_data[y - 1][x]);
        }
        if y != self.get_max_y() {
            neighbor_heights.push(self.map_data[y + 1][x]);
        }
        neighbor_heights
    }
    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let my_height = self.map_data[y][x];
        for neighbor_height in self.get_neighbor_heights(x, y).iter() {
            if my_height >= *neighbor_height {
                return false;
            }
        }
        true
    }
}
fn parse_input(puzzle_input: String) -> HeightMap {
    HeightMap {
        map_data: puzzle_input
            .trim()
            .split("\n")
            .map(|line| {
                line.chars()
                    .map(|n| n.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>(),
    }
}

pub(crate) fn solve_part_1(puzzle_input: String) -> u64 {
    let height_map = parse_input(puzzle_input);
    let mut risk_level_sum: u64 = 0;
    for x in 0..=height_map.get_max_x() {
        for y in 0..=height_map.get_max_y() {
            if height_map.is_low_point(x, y) {
                risk_level_sum += (height_map.map_data[y][x] + 1) as u64
            }
        }
    }
    risk_level_sum
}

pub(crate) fn solve_part_2(puzzle_input: String) -> u64 {
    0
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(9)), 560);
}

#[test]
fn test_part_2() {
    assert_eq!(solve_part_2(test_helpers::load_puzzle_input(9)), 0);
}
