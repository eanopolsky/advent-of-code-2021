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
    neighbors: HashSet<String>,
}

fn is_string_lower_case(test_string: &str) -> bool {
    let mut lower_string = String::from(test_string);
    lower_string.make_ascii_lowercase();
    test_string == lower_string
}

fn count_paths_to_end(
    current_room: &str,
    room_properties: &HashMap<String, RoomProperties>,
    visit_counts: &HashMap<&str, u64>,
    still_have_extra_time: bool,
) -> u64 {
    // println!(
    //     "Getting paths to end from {} with visit_counts = {:?} and still_have_extra_time = {}",
    //     current_room, visit_counts, still_have_extra_time
    // );
    let mut paths_to_end: u64 = 0;
    let mut updated_visit_counts = visit_counts.clone();
    *updated_visit_counts.get_mut(current_room).unwrap() += 1;
    for possible_next_room in room_properties.get(current_room).unwrap().neighbors.iter() {
        if possible_next_room == "end" {
            paths_to_end += 1;
            continue;
        }
        if possible_next_room == "start" {
            continue;
        }

        let possible_next_room_type: &CaveSize =
            &room_properties.get(possible_next_room).unwrap().size;
        let possible_next_room_visit_count = *visit_counts.get(&possible_next_room[..]).unwrap();
        if *possible_next_room_type == CaveSize::Small
            && possible_next_room_visit_count > 0
            && !still_have_extra_time
        {
            continue;
        }
        let paths_from_possible_next_room = count_paths_to_end(
            possible_next_room,
            room_properties,
            &updated_visit_counts,
            still_have_extra_time
                && (*possible_next_room_type == CaveSize::Big
                    || possible_next_room_visit_count == 0),
        );
        paths_to_end += paths_from_possible_next_room;
    }
    // println!("Got paths to end from {}: {:?}", current_room, paths_to_end);
    paths_to_end
}

fn parse_room_properties(puzzle_input: String) -> HashMap<String, RoomProperties> {
    let room_pair_iter = puzzle_input.trim().split("\n").map(|line| {
        let mut cave_iter = line.split("-");
        (cave_iter.next().unwrap(), cave_iter.next().unwrap())
    });
    let mut rooms: HashMap<String, RoomProperties> = HashMap::new();
    for room_pair in room_pair_iter {
        for (first_room, second_room) in [room_pair, (room_pair.1, room_pair.0)] {
            match rooms.get_mut(first_room) {
                None => {
                    rooms.insert(
                        String::from(first_room),
                        RoomProperties {
                            size: match is_string_lower_case(first_room) {
                                true => CaveSize::Small,
                                false => CaveSize::Big,
                            },
                            neighbors: {
                                let mut neighbors: HashSet<String> = HashSet::new();
                                neighbors.insert(String::from(second_room));
                                neighbors
                            },
                        },
                    );
                }
                Some(existing_room_properties) => {
                    existing_room_properties
                        .neighbors
                        .insert(String::from(second_room));
                }
            }
        }
    }
    rooms
}

fn initialize_visit_counts(
    room_properties: &HashMap<String, RoomProperties>,
) -> HashMap<&str, u64> {
    let mut visit_counts: HashMap<&str, u64> = HashMap::new();
    for room_label in room_properties.keys() {
        if room_label == "start" {
            visit_counts.insert(room_label, 1);
        } else {
            visit_counts.insert(room_label, 0);
        }
    }
    visit_counts
}

pub(crate) fn solve_part_1(puzzle_input: String) -> u64 {
    let room_properties = parse_room_properties(puzzle_input);
    // println!("Room properties: {:?}", room_properties);
    let visit_counts = initialize_visit_counts(&room_properties);
    count_paths_to_end("start", &room_properties, &visit_counts, false)
}

pub(crate) fn solve_part_2(puzzle_input: String) -> u64 {
    let room_properties = parse_room_properties(puzzle_input);
    // println!("Room properties: {:?}", room_properties);
    let visit_counts = initialize_visit_counts(&room_properties);
    count_paths_to_end("start", &room_properties, &visit_counts, true)
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
