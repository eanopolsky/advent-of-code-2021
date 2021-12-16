use std::collections::HashMap;
use std::convert::TryInto;

struct CavernMap {
    risk_levels: HashMap<(i64, i64), i64>,
    min_cost_to_origin: HashMap<(i64, i64), i64>,
    bottom_right: (i64, i64),
    locations_touched_by_last_propagation: Vec<(i64, i64)>,
}

impl CavernMap {
    fn from(text_map: String) -> Self {
        let mut new_cavern_map = CavernMap {
            risk_levels: HashMap::new(),
            min_cost_to_origin: HashMap::new(),
            bottom_right: (0, 0),
            locations_touched_by_last_propagation: vec![(0, 0)],
        };
        for (y, line) in text_map.trim().split("\n").enumerate() {
            for (x, risk_level) in line.chars().enumerate() {
                new_cavern_map.risk_levels.insert(
                    (x.try_into().unwrap(), y.try_into().unwrap()),
                    risk_level.to_digit(10).unwrap().into(),
                );
                if x != 0 || y != 0 {
                    new_cavern_map
                        .min_cost_to_origin
                        .insert((x.try_into().unwrap(), y.try_into().unwrap()), i64::MAX);
                }
            }
        }
        new_cavern_map.min_cost_to_origin.insert((0, 0), 0);
        let max_x = new_cavern_map
            .risk_levels
            .keys()
            .map(|(x, _y)| x)
            .max()
            .unwrap();
        let max_y = new_cavern_map
            .risk_levels
            .keys()
            .map(|(_x, y)| y)
            .max()
            .unwrap();
        new_cavern_map.bottom_right = (*max_x, *max_y);

        new_cavern_map
    }

    fn get_neighbors(&self, (x, y): &(i64, i64)) -> Vec<(i64, i64)> {
        let possible_neighbors = [(x - 1, *y), (x + 1, *y), (*x, y - 1), (*x, y + 1)];
        // possible_neighbors
        //     .iter()
        //     .filter(|possible_neighbor| self.risk_levels.contains_key(possible_neighbor))
        //     .map(|neighbor| *neighbor)
        //     .collect::<Vec<(i64, i64)>>()
        possible_neighbors
            .iter()
            .filter(|(x, y)| {
                *x >= 0 && *x <= self.bottom_right.0 && *y >= 0 && *y <= self.bottom_right.1
            })
            .map(|neighbor| *neighbor)
            .collect::<Vec<(i64, i64)>>()
    }

    fn get_bottom_right(&self) -> (i64, i64) {
        self.bottom_right
    }

    // fn get_risk_level(&self, (x, y): &(i64, i64)) -> i64 {
    //     *self.risk_levels.get(&(*x, *y)).unwrap()
    // }

    // fn max_cost(&self) -> i64 {
    //     *self.min_cost_to_origin.values().max().unwrap()
    // }

    fn total_cost(&self) -> i64 {
        self.min_cost_to_origin.values().sum::<i64>()
    }

    fn propagate_costs(&mut self) {
        // let mut new_min_cost_to_origin: HashMap<(i64, i64), i64> = HashMap::new();
        let mut new_locations_touched: Vec<(i64, i64)> = Vec::new();
        // update following logic to only update neighbors of
        // locations touched by last propagation:
        for previously_touched_location in &self.locations_touched_by_last_propagation {
            let cost = *self
                .min_cost_to_origin
                .get(&previously_touched_location)
                .unwrap();
            for neighbor_location in self.get_neighbors(&previously_touched_location) {
                let neighbor_risk_level = self.risk_levels.get(&neighbor_location).unwrap();
                let neighbor_minimum_cost =
                    self.min_cost_to_origin.get_mut(&neighbor_location).unwrap();
                if cost + neighbor_risk_level < *neighbor_minimum_cost {
                    *neighbor_minimum_cost = cost + neighbor_risk_level;
                    new_locations_touched.push(neighbor_location);
                }
            }
        }
        self.locations_touched_by_last_propagation = new_locations_touched;
    }

    fn enlarge_cavern(&mut self) {
        // for part 2
        let mut enlarged_risk_levels: HashMap<(i64, i64), i64> = HashMap::new();
        for (location, risk_level) in self.risk_levels.iter() {
            for x_offset in 0..=4 {
                for y_offset in 0..=4 {
                    let new_location = (
                        location.0 + x_offset * (self.bottom_right.0 + 1),
                        location.1 + y_offset * (self.bottom_right.1 + 1),
                    );
                    let new_risk_level = (risk_level - 1 + x_offset + y_offset) % 9 + 1;
                    enlarged_risk_levels.insert(new_location, new_risk_level);
                }
            }
        }
        self.risk_levels = enlarged_risk_levels;
        let mut enlarged_min_cost_to_origin: HashMap<(i64, i64), i64> = HashMap::new();
        for location in self.risk_levels.keys() {
            enlarged_min_cost_to_origin.insert(*location, i64::MAX);
        }
        enlarged_min_cost_to_origin.insert((0, 0), 0);
        self.min_cost_to_origin = enlarged_min_cost_to_origin;
        let max_x = self.risk_levels.keys().map(|(x, _y)| x).max().unwrap();
        let max_y = self.risk_levels.keys().map(|(_x, y)| y).max().unwrap();
        self.bottom_right = (*max_x, *max_y);
        println!("Bottom right is now {:?}", self.bottom_right);
    }
}

pub(crate) fn solve_part_1(puzzle_input: String) -> String {
    let mut cavern_map = CavernMap::from(puzzle_input);
    let minimum_passes = cavern_map.bottom_right.0 + cavern_map.bottom_right.1;
    for _ in 1..=minimum_passes {
        println!("Propagating costs.");
        cavern_map.propagate_costs();
    }

    // println!(
    //     "No minimum costs are i64::MAX. Largest is now {}",
    //     cavern_map.max_cost()
    // );

    let mut last_total_cost = i64::MAX;
    while last_total_cost > cavern_map.total_cost() {
        last_total_cost = cavern_map.total_cost();
        cavern_map.propagate_costs();
        println!("Total costs are now {}", cavern_map.total_cost());
    }
    cavern_map
        .min_cost_to_origin
        .get(&cavern_map.get_bottom_right())
        .unwrap()
        .to_string()
    //    "".to_string()
}

pub(crate) fn solve_part_2(puzzle_input: String) -> String {
    let mut cavern_map = CavernMap::from(puzzle_input);
    cavern_map.enlarge_cavern();
    let minimum_passes = cavern_map.bottom_right.0 + cavern_map.bottom_right.1;
    for _ in 1..=minimum_passes {
        println!("Propagating costs.");
        cavern_map.propagate_costs();
    }
    // println!(
    //     "No minimum costs are i64::MAX. Largest is now {}",
    //     cavern_map.max_cost()
    // );
    let mut last_total_cost = i64::MAX;
    while last_total_cost > cavern_map.total_cost() {
        last_total_cost = cavern_map.total_cost();
        cavern_map.propagate_costs();
        println!("Total costs are now {}", cavern_map.total_cost());
    }
    cavern_map
        .min_cost_to_origin
        .get(&cavern_map.get_bottom_right())
        .unwrap()
        .to_string()
    //    "".to_string()
}
