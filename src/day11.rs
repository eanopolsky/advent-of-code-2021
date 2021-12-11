use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
struct OctopusMap {
    hash_map: HashMap<(i8, i8), Octopus>,
}

#[derive(Debug)]
struct Octopus {
    energy_level: u8,
    has_flashed_this_step: bool,
}

impl Octopus {
    fn increment_energy_level(&mut self) {
        self.energy_level += 1;
    }
    fn end_of_step_cleanup(&mut self) {
        if self.has_flashed_this_step {
            self.has_flashed_this_step = false;
            self.energy_level = 0;
        }
    }

    /// Returns true if the octopus successfully flashed.
    fn attempt_flash(&mut self) -> bool {
        if self.has_flashed_this_step {
            return false;
        }
        if self.energy_level > 9 {
            self.has_flashed_this_step = true;
            return true;
        }
        false
    }
}

impl fmt::Display for Octopus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.energy_level {
            0..=9 => write!(f, "{}", self.energy_level),
            10..=18 => write!(f, "R"),  // "R"eady to flash
            19..=255 => write!(f, "?"), // panic!("Invalid energy level"),
        }
    }
}

impl fmt::Display for OctopusMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.get_min_y()..=self.get_max_y() {
            for x in self.get_min_x()..=self.get_max_x() {
                match self.hash_map.get(&(x, y)) {
                    None => write!(f, " ")?,
                    Some(octopus) => write!(f, "{}", octopus)?,
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl OctopusMap {
    fn create_octopus(&mut self, x: i8, y: i8, energy_level: u8) -> () {
        self.hash_map.insert(
            (x, y),
            Octopus {
                energy_level,
                has_flashed_this_step: false,
            },
        );
    }
    fn get_octopus_count(&self) -> usize {
        self.hash_map.len()
    }
    fn get_min_x(&self) -> i8 {
        self.hash_map
            .keys()
            .map(|coordinates| coordinates.0)
            .min()
            .unwrap()
    }
    fn get_max_x(&self) -> i8 {
        self.hash_map
            .keys()
            .map(|coordinates| coordinates.0)
            .max()
            .unwrap()
    }
    fn get_min_y(&self) -> i8 {
        self.hash_map
            .keys()
            .map(|coordinates| coordinates.1)
            .min()
            .unwrap()
    }
    fn get_max_y(&self) -> i8 {
        self.hash_map
            .keys()
            .map(|coordinates| coordinates.1)
            .max()
            .unwrap()
    }
    fn mass_charge_octopodes(&mut self) {
        for octopus in self.hash_map.values_mut() {
            (*octopus).increment_energy_level();
        }
    }
    fn simulate_step(&mut self) -> u64 {
        // println!("Beginning step simulation. Starting octopus map:\n{}", self);
        self.mass_charge_octopodes();
        // println!("After mass charge:\n{}", self);
        let mut flash_count: u64 = 0;
        let mut an_octopus_flashed_this_iteration = true;
        while an_octopus_flashed_this_iteration {
            an_octopus_flashed_this_iteration = false;
            let mut flashed_coordinates: Vec<(i8, i8)> = Vec::new();
            for (coordinates, octopus) in &mut self.hash_map {
                let octopus_successfully_flashed = octopus.attempt_flash();
                if !octopus_successfully_flashed {
                    continue;
                }
                flash_count += 1;
                an_octopus_flashed_this_iteration = true;
                flashed_coordinates.push((coordinates.0 - 1, coordinates.1 - 1));
                flashed_coordinates.push((coordinates.0 - 1, coordinates.1));
                flashed_coordinates.push((coordinates.0 - 1, coordinates.1 + 1));
                flashed_coordinates.push((coordinates.0, coordinates.1 - 1));
                flashed_coordinates.push((coordinates.0, coordinates.1));
                flashed_coordinates.push((coordinates.0, coordinates.1 + 1));
                flashed_coordinates.push((coordinates.0 + 1, coordinates.1 - 1));
                flashed_coordinates.push((coordinates.0 + 1, coordinates.1));
                flashed_coordinates.push((coordinates.0 + 1, coordinates.1 + 1));
            }
            // println!(
            //     "The following locations received flashes: {:?}",
            //     flashed_coordinates
            // );
            for flashed_location in flashed_coordinates.iter() {
                match self.hash_map.get_mut(flashed_location) {
                    None => {
                        // println!(
                        //     "No octopus found at {:?} to receive flash.",
                        //     flashed_location
                        // );
                        continue;
                    }
                    Some(octopus_neighbor) => {
                        // println!("Octopus found at {:?} to receive flash.", flashed_location);
                        // println!(
                        //     "Flashed octopus before incrementing energy level: {}",
                        //     octopus_neighbor
                        // );
                        octopus_neighbor.increment_energy_level();
                        // println!(
                        //     "Flashed octopus after incrementing energy level: {}",
                        //     octopus_neighbor
                        // );
                    }
                }
            }
            // println!("After flash charges applied:\n{}", self);
        }
        for octopus in self.hash_map.values_mut() {
            octopus.end_of_step_cleanup();
        }
        flash_count
    }
}

fn load_octopodes(puzzle_input: String) -> OctopusMap {
    let mut my_octopus_map = OctopusMap {
        hash_map: HashMap::<(i8, i8), Octopus>::new(),
    };
    for (y, octopus_line) in puzzle_input.trim().split("\n").enumerate() {
        for (x, octopus_char) in octopus_line.chars().enumerate() {
            my_octopus_map.create_octopus(
                x as i8,
                y as i8,
                octopus_char.to_digit(10).unwrap() as u8,
            );
        }
    }
    my_octopus_map
}

pub(crate) fn solve_part_1(puzzle_input: String) -> u64 {
    let mut my_octopus_map = load_octopodes(puzzle_input);
    // println!("Initial map:\n{}", my_octopus_map);
    let mut total_flashes: u64 = 0;
    for _step in 1..=100 {
        total_flashes += my_octopus_map.simulate_step();
        // println!("After executing step {}:\n{}", step, my_octopus_map);
    }
    total_flashes
}

pub(crate) fn solve_part_2(puzzle_input: String) -> u64 {
    let mut my_octopus_map = load_octopodes(puzzle_input);
    let population_size: u64 = my_octopus_map.get_octopus_count() as u64;
    let mut steps_performed: u64 = 0;
    loop {
        steps_performed += 1;
        if my_octopus_map.simulate_step() == population_size {
            break;
        }
    }
    steps_performed
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(11)), 1705);
}

#[test]
fn test_part_2() {
    assert_eq!(solve_part_2(test_helpers::load_puzzle_input(11)), 265);
}
