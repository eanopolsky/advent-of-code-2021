use std::collections::HashSet;

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
        for (neighbor_x, neighbor_y) in self.get_neighbor_coordinates(x, y) {
            neighbor_heights.push(self.map_data[neighbor_y][neighbor_x]);
        }
        neighbor_heights
    }
    fn get_neighbor_coordinates(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbor_coordinates: Vec<(usize, usize)> = Vec::new();
        if x != 0 {
            neighbor_coordinates.push((x - 1, y));
        }
        if x != self.get_max_x() {
            neighbor_coordinates.push((x + 1, y));
        }
        if y != 0 {
            neighbor_coordinates.push((x, y - 1));
        }
        if y != self.get_max_y() {
            neighbor_coordinates.push((x, y + 1));
        }
        neighbor_coordinates
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
    fn get_height(&self, x: usize, y: usize) -> u8 {
        self.map_data[y][x]
    }
    fn find_basin_size(&self, x: usize, y: usize) -> usize {
        // x, y should be the coordinates of a low point
        // first find list of basin member coordinates. Then return its length.
        let mut basin_members: HashSet<(usize, usize)> = HashSet::new();
        let mut new_neighbors: HashSet<(usize, usize)> = HashSet::new();
        new_neighbors.insert((x, y));
        loop {
            if new_neighbors.len() == 0 {
                break;
            }
            for new_neighbor in new_neighbors.iter() {
                basin_members.insert(*new_neighbor);
            }
            new_neighbors.clear();
            for basin_member in basin_members.iter() {
                for neighbor in self
                    .get_neighbor_coordinates(basin_member.0, basin_member.1)
                    .iter()
                {
                    if self.get_height(neighbor.0, neighbor.1) == 9 {
                        continue;
                    }
                    if basin_members.contains(neighbor) == true {
                        continue;
                    }
                    new_neighbors.insert(*neighbor);
                }
            }
        }
        basin_members.len()
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
    let height_map = parse_input(puzzle_input);
    let mut basin_sizes: Vec<usize> = Vec::new();
    for x in 0..=height_map.get_max_x() {
        for y in 0..=height_map.get_max_y() {
            if !height_map.is_low_point(x, y) {
                continue;
            }
            basin_sizes.push(height_map.find_basin_size(x, y));
        }
    }
    basin_sizes.sort_by(|a, b| b.partial_cmp(a).unwrap());
    basin_sizes[0] as u64 * basin_sizes[1] as u64 * basin_sizes[2] as u64
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(9)), 560);
}

#[test]
fn test_part_2() {
    assert_eq!(solve_part_2(test_helpers::load_puzzle_input(9)), 959136);
}
