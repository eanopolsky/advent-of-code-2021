use std::collections::HashSet;

pub(crate) fn solve_part_1(puzzle_input: String) -> u64 {
    let mut puzzle_parts = puzzle_input.trim().split("\n\n");
    let dot_coordinates_list = puzzle_parts.next().unwrap();
    let fold_instructions_list = puzzle_parts.next().unwrap();

    let dot_coordinates_iter = dot_coordinates_list.split("\n").map(|line| {
        let mut ordinates_iter = line.split(",").map(|n| n.parse::<i64>().unwrap());
        (
            ordinates_iter.next().unwrap(), //x
            ordinates_iter.next().unwrap(), //y
        )
    });

    let mut dot_coordinates = HashSet::<(i64, i64)>::new();
    for coordinates in dot_coordinates_iter {
        dot_coordinates.insert(coordinates);
    }

    let fold_instructions = fold_instructions_list
        .split("\n")
        .map(|line| {
            let mut line_iter = line.split("=");
            (
                line_iter.next().unwrap(),
                line_iter.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<(&str, i64)>>();
    // println!(
    //     "Dot coordinates:\n{:?}\nFold instructions:\n{:?}",
    //     dot_coordinates, fold_instructions
    // );

    // for fold_instruction in fold_instructions {
    let fold_instruction = fold_instructions[0];
    let mut new_dot_coordinates = HashSet::<(i64, i64)>::new();
    if fold_instruction.0 == "fold along y" {
        for old_dot_coordinate_pair in &dot_coordinates {
            if old_dot_coordinate_pair.1 < fold_instruction.1 {
                new_dot_coordinates.insert(*old_dot_coordinate_pair);
            } else if old_dot_coordinate_pair.1 > fold_instruction.1 {
                new_dot_coordinates.insert((
                    old_dot_coordinate_pair.0,
                    2 * fold_instruction.1 - old_dot_coordinate_pair.1,
                ));
            } else {
                panic!("Invalid coordinate found");
            }
        }
    } else if fold_instruction.0 == "fold along x" {
        for old_dot_coordinate_pair in &dot_coordinates {
            if old_dot_coordinate_pair.0 < fold_instruction.1 {
                new_dot_coordinates.insert(*old_dot_coordinate_pair);
            } else if old_dot_coordinate_pair.0 > fold_instruction.1 {
                new_dot_coordinates.insert((
                    2 * fold_instruction.1 - old_dot_coordinate_pair.0,
                    old_dot_coordinate_pair.1,
                ));
            } else {
                panic!("Invalid coordinate found");
            }
        }
    } else {
        panic!("Invalid fold instruction");
    }
    // }
    new_dot_coordinates.len() as u64
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(13)), 810);
}
