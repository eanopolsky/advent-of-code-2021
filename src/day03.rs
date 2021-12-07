pub(crate) fn solve_part_1(puzzle_input: String) -> u32 {
    let diagnostic_report = puzzle_input
        .split("\n")
        .filter(|report_number| (report_number.len() != 0));
    let mut report_size = 0;
    let mut bit_counts: [u32; 12] = [0; 12];
    for report_entry in diagnostic_report {
        report_size += 1;
        for (i, report_char) in report_entry.chars().enumerate() {
            if report_char == '1' {
                bit_counts[i] += 1;
            }
        }
    }
    let mut gamma_rate = 0;
    for count in bit_counts {
        if count * 2 > report_size {
            gamma_rate += 1;
        }
        gamma_rate *= 2;
    }
    gamma_rate /= 2;
    let epsilon_rate = 2_u32.pow(12) - 1 - gamma_rate;
    epsilon_rate * gamma_rate
}

pub(crate) fn solve_part_2(puzzle_input: String) -> u32 {
    let diagnostic_report: Vec<&str> = puzzle_input
        .split("\n")
        .filter(|report_number| -> bool { report_number.len() != 0 })
        .collect();
    let mut oxygen_generator_possibilities = diagnostic_report.clone();
    for bit_position in 0..12 {
        oxygen_generator_possibilities =
            trim_diagnostic_report(oxygen_generator_possibilities, bit_position, true);
        if oxygen_generator_possibilities.len() == 1 {
            break;
        }
    }

    let mut co2_scrubber_possibilities = diagnostic_report.clone();
    for bit_position in 0..12 {
        co2_scrubber_possibilities =
            trim_diagnostic_report(co2_scrubber_possibilities, bit_position, false);
        if co2_scrubber_possibilities.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(oxygen_generator_possibilities[0], 2).unwrap()
        * u32::from_str_radix(co2_scrubber_possibilities[0], 2).unwrap()
}

fn trim_diagnostic_report(
    report: Vec<&str>,
    bit_position: usize,
    keep_most_common: bool,
) -> Vec<&str> {
    let report_length = report.len();
    let mut one_count_at_bit_position = 0;
    for report_element in report.iter() {
        if report_element.chars().collect::<Vec<char>>()[bit_position] == '1' {
            one_count_at_bit_position += 1;
        }
    }

    let keep_ones: bool = (one_count_at_bit_position * 2 >= report_length && keep_most_common)
        || (one_count_at_bit_position * 2 < report_length && !keep_most_common);

    let mut trimmed_report: Vec<&str> = Vec::new();
    for report_element in report.iter() {
        let char_at_bit_position = report_element.chars().collect::<Vec<char>>()[bit_position];
        if (char_at_bit_position == '1' && keep_ones) || (char_at_bit_position == '0' && !keep_ones)
        {
            trimmed_report.push(report_element);
        }
    }
    trimmed_report
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(3)), 1092896);
}

#[test]
fn test_part_2() {
    assert_eq!(solve_part_2(test_helpers::load_puzzle_input(3)), 4672151);
}
