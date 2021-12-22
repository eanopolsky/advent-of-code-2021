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

#[derive(Debug)]
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
