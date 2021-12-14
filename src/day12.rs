use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum CaveSize {
    Small,
    Big,
}

#[derive(Debug)]
struct RoomProperties {
    size: CaveSize,
    neighbors: Vec<usize>,
}

fn is_string_lower_case(test_string: &str) -> bool {
    let mut lower_string = String::from(test_string);
    lower_string.make_ascii_lowercase();
    test_string == lower_string
}

fn count_paths_to_end(
    current_room_offset: &usize,
    start_room_offset: &usize,
    end_room_offset: &usize,
    all_room_properties: &Vec<RoomProperties>,
    visit_counts: &Vec<usize>,
    still_have_extra_time: bool,
) -> u64 {
    let mut paths_to_end: u64 = 0;
    let mut updated_visit_counts = visit_counts.clone();
    updated_visit_counts[*current_room_offset] += 1;
    for possible_next_room_offset in all_room_properties[*current_room_offset].neighbors.iter() {
        if possible_next_room_offset == end_room_offset {
            paths_to_end += 1;
            continue;
        }
        if possible_next_room_offset == start_room_offset {
            continue;
        }

        let possible_next_room_type: &CaveSize =
            &all_room_properties[*possible_next_room_offset].size;
        let possible_next_room_visit_count = visit_counts[*possible_next_room_offset];
        if *possible_next_room_type == CaveSize::Small
            && possible_next_room_visit_count > 0
            && !still_have_extra_time
        {
            continue;
        }
        let paths_from_possible_next_room = count_paths_to_end(
            possible_next_room_offset,
            start_room_offset,
            end_room_offset,
            all_room_properties,
            &updated_visit_counts,
            still_have_extra_time
                && (*possible_next_room_type == CaveSize::Big
                    || possible_next_room_visit_count == 0),
        );
        paths_to_end += paths_from_possible_next_room;
    }
    paths_to_end
}

fn parse_room_properties(puzzle_input: &str) -> (Vec<RoomProperties>, usize, usize) {
    let mut room_label_set: HashSet<&str> = HashSet::new();
    for room_label in puzzle_input.trim().split(&['\n', '-'][..]) {
        room_label_set.insert(room_label);
    }
    let mut room_to_offset_map: HashMap<&str, usize> = HashMap::new();
    let mut all_room_properties: Vec<RoomProperties> = Vec::new();
    let mut offset: usize = 0;
    for room_label in room_label_set {
        room_to_offset_map.insert(room_label, offset);
        all_room_properties.push(RoomProperties {
            size: match is_string_lower_case(room_label) {
                true => CaveSize::Small,
                false => CaveSize::Big,
            },
            neighbors: Vec::new(),
        });
        offset += 1;
    }

    for room_pair in puzzle_input.trim().split("\n").map(|line| {
        let mut cave_iter = line.split("-");
        (cave_iter.next().unwrap(), cave_iter.next().unwrap())
    }) {
        all_room_properties[*room_to_offset_map.get(room_pair.0).unwrap()]
            .neighbors
            .push(*room_to_offset_map.get(room_pair.1).unwrap());
        all_room_properties[*room_to_offset_map.get(room_pair.1).unwrap()]
            .neighbors
            .push(*room_to_offset_map.get(room_pair.0).unwrap());
    }
    (
        all_room_properties,
        *room_to_offset_map.get("start").unwrap(),
        *room_to_offset_map.get("end").unwrap(),
    )
}

pub(crate) fn solve_part_1(puzzle_input: String) -> u64 {
    let (all_room_properties, start_offset, end_offset) = parse_room_properties(&puzzle_input);
    let visit_counts = vec![0 as usize; all_room_properties.len()];
    count_paths_to_end(
        &start_offset,
        &start_offset,
        &end_offset,
        &all_room_properties,
        &visit_counts,
        false,
    )
}

pub(crate) fn solve_part_2(puzzle_input: String) -> u64 {
    let (all_room_properties, start_offset, end_offset) = parse_room_properties(&puzzle_input);
    let visit_counts = vec![0 as usize; all_room_properties.len()];
    count_paths_to_end(
        &start_offset,
        &start_offset,
        &end_offset,
        &all_room_properties,
        &visit_counts,
        true,
    )
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(12)), 5212);
}

#[test]
fn test_part_2() {
    assert_eq!(solve_part_2(test_helpers::load_puzzle_input(12)), 134862);
}
