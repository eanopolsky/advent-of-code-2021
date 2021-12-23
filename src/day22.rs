use std::collections::HashSet;
use std::ops::Range;

pub(crate) fn solve_part_1(puzzle_input: String) -> String {
    let reboot_steps = parse_reboot_steps(&puzzle_input);
    // for reboot_step in reboot_steps {
    //     println!("{:?}", reboot_step);
    // }
    let mut on_cubes: HashSet<(i32, i32, i32)> = HashSet::new();
    for reboot_step in reboot_steps.iter() {
        if (reboot_step.x_range.start < -50 && reboot_step.x_range.end <= -50)
            || (reboot_step.y_range.start < -50 && reboot_step.y_range.end <= -50)
            || (reboot_step.z_range.start < -50 && reboot_step.z_range.end <= -50)
            || (reboot_step.x_range.start > 50 && reboot_step.x_range.end >= 50)
            || (reboot_step.y_range.start > 50 && reboot_step.y_range.end >= 50)
            || (reboot_step.z_range.start > 50 && reboot_step.z_range.end >= 50)
        {
            continue;
        }
        for x in reboot_step.x_range.start..reboot_step.x_range.end {
            if x < -50 || x > 50 {
                continue;
            }
            for y in reboot_step.y_range.start..reboot_step.y_range.end {
                if y < -50 || y > 50 {
                    continue;
                }
                for z in reboot_step.z_range.start..reboot_step.z_range.end {
                    if z < -50 || z > 50 {
                        continue;
                    }
                    if reboot_step.turn_on {
                        on_cubes.insert((x, y, z));
                    } else {
                        on_cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    on_cubes.len().to_string()
}

pub(crate) fn solve_part_2(puzzle_input: String) -> String {
    let mut reboot_steps = parse_reboot_steps(&puzzle_input);
    // // steps that overlap the origin in the x coordinate
    // for reboot_step in reboot_steps {
    //     if reboot_step.x_range.start < 0 && reboot_step.x_range.end > 0 {
    //         println!("{:?}", reboot_step);
    //     }
    // }

    // steps that are overlapped by subsequent steps
    // let mut last_sum_of_all_on_cubes = sum_of_all_on_cubes(&reboot_steps);
    // println!("Got reboot steps: {:?}", reboot_steps);
    loop {
        let mut new_reboot_steps: Vec<RebootStep> = Vec::new();
        let mut found_any_overlaps = false;
        for (i, reboot_step) in reboot_steps.iter().enumerate() {
            // println!(
            //     "Comparing reboot step at index {} against subsequent steps",
            //     i
            // );
            // println!("\nCurrent step is 'on' type: {}", reboot_step.turn_on);
            // println!("Examining step: {:#?}", reboot_step);
            let mut found_overlap = false;
            for subsequent_step in reboot_steps[(i + 1)..].iter() {
                // println!(
                //     "Considering subsequent step at offset {}: {:?}",
                //     j + i + 1,
                //     subsequent_step
                // );
                if reboot_step_cuboids_overlap(&reboot_step, &subsequent_step) {
                    found_overlap = true;
                    found_any_overlaps = true;
                    let mut difference = reboot_step_difference(&reboot_step, subsequent_step);
                    // match difference.len() {
                    //     0 | 3 => (),
                    //     _ => println!("Got a potentially remarkable number of reboot steps."),
                    // }
                    // println!("It overlaps with subsequent step: {:#?}", subsequent_step);
                    // println!("Reboot steps representing cubes present in this step but not the identified subsequent step: {:#?}\n", difference);
                    new_reboot_steps.append(&mut difference);
                    break;
                }
            }
            if !found_overlap {
                // println!("No subsequent overlapping steps were found.\n");
                new_reboot_steps.push(reboot_step.clone())
            }
        }
        reboot_steps = new_reboot_steps;
        if !found_any_overlaps {
            break;
        }
    }
    sum_of_all_on_cubes(&reboot_steps).to_string()
}

fn reboot_step_cuboids_overlap(a: &RebootStep, b: &RebootStep) -> bool {
    // println!("Determining whether cuboids overlap: {:?}, {:?}", a, b);
    ranges_overlap(&a.x_range, &b.x_range)
        && ranges_overlap(&a.y_range, &b.y_range)
        && ranges_overlap(&a.z_range, &b.z_range)
}

fn ranges_overlap(a: &Range<i32>, b: &Range<i32>) -> bool {
    let result = (a.start >= b.start && a.start < b.end)
        || (a.end > b.start && a.end <= b.end)
        || (a.start <= b.start && a.end >= b.end);
    // println!("Ranges {:?} and {:?} overlap: {}", a, b, result);
    result
}

/// Produces a vector of up to three reboot steps containing the cubes
/// affected by `first` and not affected by `second`.
fn reboot_step_difference(first: &RebootStep, second: &RebootStep) -> Vec<RebootStep> {
    if !reboot_step_cuboids_overlap(first, second) {
        panic!("This function only works on overlapping steps");
    }
    let mut result: Vec<RebootStep> = Vec::new();
    if first.x_range.start < second.x_range.start {
        result.push(RebootStep {
            turn_on: first.turn_on,
            x_range: Range::<i32> {
                start: first.x_range.start,
                end: second.x_range.start,
            },
            y_range: first.y_range.clone(),
            z_range: first.z_range.clone(),
        });
    }
    if second.x_range.end < first.x_range.end {
        result.push(RebootStep {
            turn_on: first.turn_on,
            x_range: Range::<i32> {
                start: second.x_range.end,
                end: first.x_range.end,
            },
            y_range: first.y_range.clone(),
            z_range: first.z_range.clone(),
        });
    }
    let overlap_x_range = Range::<i32> {
        start: match first.x_range.start < second.x_range.start {
            true => second.x_range.start,
            false => first.x_range.start,
        },
        end: match first.x_range.end < second.x_range.end {
            true => first.x_range.end,
            false => second.x_range.end,
        },
    };
    if first.y_range.start < second.y_range.start {
        result.push(RebootStep {
            turn_on: first.turn_on,
            x_range: overlap_x_range.clone(),
            y_range: Range::<i32> {
                start: first.y_range.start,
                end: second.y_range.start,
            },
            z_range: first.z_range.clone(),
        });
    }
    if second.y_range.end < first.y_range.end {
        result.push(RebootStep {
            turn_on: first.turn_on,
            x_range: overlap_x_range.clone(),
            y_range: Range::<i32> {
                start: second.y_range.end,
                end: first.y_range.end,
            },
            z_range: first.z_range.clone(),
        });
    }
    let overlap_y_range = Range::<i32> {
        start: match first.y_range.start < second.y_range.start {
            true => second.y_range.start,
            false => first.y_range.start,
        },
        end: match first.y_range.end < second.y_range.end {
            true => first.y_range.end,
            false => second.y_range.end,
        },
    };
    if first.z_range.start < second.z_range.start {
        result.push(RebootStep {
            turn_on: first.turn_on,
            x_range: overlap_x_range.clone(),
            y_range: overlap_y_range.clone(),
            z_range: Range::<i32> {
                start: first.z_range.start,
                end: second.z_range.start,
            },
        });
    }
    if second.z_range.end < first.z_range.end {
        result.push(RebootStep {
            turn_on: first.turn_on,
            x_range: overlap_x_range.clone(),
            y_range: overlap_y_range.clone(),
            z_range: Range::<i32> {
                start: second.z_range.end,
                end: first.z_range.end,
            },
        });
    }
    result
}

/// Returns a count of all cubes that are turned on in all reboot
/// steps. Does not take into account cubes that may have been turned
/// on twice, or cubes that may have been turned off and back on.
fn sum_of_all_on_cubes(reboot_steps: &Vec<RebootStep>) -> u64 {
    let mut result: u64 = 0;
    for reboot_step in reboot_steps.iter() {
        if !reboot_step.turn_on {
            continue;
        }
        result += reboot_step.x_range.len() as u64
            * reboot_step.y_range.len() as u64
            * reboot_step.z_range.len() as u64;
    }
    result
}

#[derive(Debug, Clone)]
struct RebootStep {
    turn_on: bool, // true when the step is "on", false otherwise
    x_range: Range<i32>,
    y_range: Range<i32>,
    z_range: Range<i32>,
}

fn parse_reboot_steps(puzzle_input: &str) -> Vec<RebootStep> {
    puzzle_input
        .trim()
        .split("\n")
        .map(|line| {
            let mut line_iter = line.split(" ");
            let turn_on = match line_iter.next().unwrap() {
                "on" => true,
                "off" => false,
                _ => panic!("Invalid operation on cuboid region"),
            };
            let mut range_iter = line_iter.next().unwrap().split(",").map(|range_text| {
                let mut endpoint_iter = range_text
                    .trim_matches(&['x', 'y', 'z', '='] as &[_])
                    .split("..");
                let first_endpoint = endpoint_iter.next().unwrap().parse::<i32>().unwrap();
                let second_endpoint = endpoint_iter.next().unwrap().parse::<i32>().unwrap();
                Range::<i32> {
                    start: match first_endpoint < second_endpoint {
                        true => first_endpoint,
                        false => second_endpoint,
                    },
                    end: match first_endpoint < second_endpoint {
                        true => second_endpoint + 1,
                        false => first_endpoint + 1,
                    }, // +1 because problem ranges are closed but Rust
                       // ranges are half open.
                }
            });
            RebootStep {
                turn_on,
                x_range: range_iter.next().unwrap(),
                y_range: range_iter.next().unwrap(),
                z_range: range_iter.next().unwrap(),
            }
        })
        .collect::<Vec<RebootStep>>()
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(22)), "658691");
}

#[test]
fn test_part_2() {
    assert_eq!(
        solve_part_2(test_helpers::load_puzzle_input(22)),
        "1228699515783640"
    );
}
