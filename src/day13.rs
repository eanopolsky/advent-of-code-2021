use std::collections::HashSet;

fn parse_input(puzzle_input: String) -> (HashSet<(i64, i64)>, Vec<(String, i64)>) {
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
                String::from(line_iter.next().unwrap()),
                line_iter.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<(String, i64)>>();
    (dot_coordinates, fold_instructions)
}

fn perform_fold(
    dot_coordinates: &HashSet<(i64, i64)>,
    fold_instruction: &(String, i64),
) -> HashSet<(i64, i64)> {
    let mut new_dot_coordinates = HashSet::<(i64, i64)>::new();
    if fold_instruction.0 == "fold along y" {
        for old_dot_coordinate_pair in dot_coordinates {
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
        for old_dot_coordinate_pair in dot_coordinates {
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
    new_dot_coordinates
}

fn render_dots(dots: &HashSet<(i64, i64)>) {
    let x_min = dots.iter().map(|tuple| tuple.0).min().unwrap();
    let x_max = dots.iter().map(|tuple| tuple.0).max().unwrap();
    let y_min = dots.iter().map(|tuple| tuple.1).min().unwrap();
    let y_max = dots.iter().map(|tuple| tuple.1).max().unwrap();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

pub(crate) fn solve_part_1(puzzle_input: String) -> u64 {
    let (dot_coordinates, fold_instructions) = parse_input(puzzle_input);
    perform_fold(&dot_coordinates, &fold_instructions[0]).len() as u64
}

pub(crate) fn solve_part_2(puzzle_input: String) -> u64 {
    let (mut dot_coordinates, fold_instructions) = parse_input(puzzle_input);
    //    let mut new_dot_coordinates = HashSet::<(i64, i64)>::new();
    for fold_instruction in fold_instructions {
        dot_coordinates = perform_fold(&dot_coordinates, &fold_instruction);
    }
    render_dots(&dot_coordinates);
    0
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(13)), 810);
}
