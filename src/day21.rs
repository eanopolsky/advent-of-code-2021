pub(crate) fn solve_part_1(puzzle_input: String) -> String {
    let (player_1_start, player_2_start) = get_starting_positions(puzzle_input);
    // println!("{} {}", player_1_start, player_2_start);

    // Player 1 landed squares will have period 5: +6, +4, +2, +0, +8 (= +20)
    // Player 2 landed squares will have period 10: +5, +3, +1, +9, +7, +5, +3, +1, +9, +7 (= +50)

    // let mut player_1_position = player_1_start;
    let mut player_1_score_loop = [
        (player_1_start + 6) % 10,
        (player_1_start + 10) % 10,
        (player_1_start + 12) % 10,
        (player_1_start + 12) % 10,
        player_1_start,
    ];
    for v in &mut player_1_score_loop {
        if *v == 0 {
            *v = 10;
        }
    }
    let player_1_score_loop = player_1_score_loop;
    let player_1_wins_after_round = first_round_with_score_at_least_1000(&player_1_score_loop);

    let mut player_2_score_loop = [
        (player_2_start + 5) % 10,
        (player_2_start + 8) % 10,
        (player_2_start + 9) % 10,
        (player_2_start + 8) % 10,
        (player_2_start + 5) % 10,
        player_2_start % 10,
        (player_2_start + 3) % 10,
        (player_2_start + 4) % 10,
        (player_2_start + 3) % 10,
        player_2_start % 10,
    ];
    for v in &mut player_2_score_loop {
        if *v == 0 {
            *v = 10;
        }
    }
    let player_2_score_loop = player_2_score_loop;
    let player_2_wins_after_round = first_round_with_score_at_least_1000(&player_2_score_loop);
    let player_1_wins = player_1_wins_after_round <= player_2_wins_after_round;
    match player_1_wins {
        true => {
            let loser_score =
                get_score_after_round(&player_2_score_loop, player_1_wins_after_round - 1);
            let die_roll_count = (2 * player_1_wins_after_round - 1) * 3;
            loser_score * die_roll_count
        }
        false => {
            let loser_score =
                get_score_after_round(&player_1_score_loop, player_2_wins_after_round);
            let die_roll_count = (2 * player_2_wins_after_round) * 3;
            loser_score * die_roll_count
        }
    }
    .to_string()
}

pub(crate) fn solve_part_2(puzzle_input: String) -> String {
    let (player_1_start, player_2_start) = get_starting_positions(puzzle_input);
    // let mut memoization_cache: HashMap<(u32, u32, u32, u32), (u64, u64)> = HashMap::new();
    let mut memoization_cache: Vec<Option<(u64, u64)>> = vec![None; 2_usize.pow(20)];
    let (player_1_win_count, player_2_win_count) =
        calculate_win_counts(player_1_start, 0, player_2_start, 0, &mut memoization_cache);
    //println!("{} {}", player_1_win_count, player_2_win_count);
    match player_1_win_count > player_2_win_count {
        true => player_1_win_count,
        false => player_2_win_count,
    }
    .to_string()
}

fn get_starting_positions(puzzle_input: String) -> (u32, u32) {
    let mut start_iter = puzzle_input.trim().split("\n").map(|line| {
        let mut line_iter = line.split("starting position: ");
        line_iter.next();
        line_iter.next().unwrap().parse::<u32>().unwrap()
    });
    (start_iter.next().unwrap(), start_iter.next().unwrap())
}

fn get_score_after_round(player_score_loop: &[u32], round: u32) -> u32 {
    let full_score_loops = round / player_score_loop.len() as u32;
    let extra_rounds = round - full_score_loops * player_score_loop.len() as u32;
    let mut score: u32 = full_score_loops * player_score_loop.iter().sum::<u32>();
    for i in 0..extra_rounds {
        score += player_score_loop[i as usize];
    }
    score
}

fn first_round_with_score_at_least_1000(player_score_loop: &[u32]) -> u32 {
    let mut round = 1000 / player_score_loop.iter().sum::<u32>() * player_score_loop.len() as u32;
    for _ in 0..player_score_loop.len() {
        if get_score_after_round(player_score_loop, round) >= 1000 {
            return round;
        }
        round += 1;
    }
    panic!("Should have found a round inside the for loop");
}

// Results are: (number of times player 1 wins, number of times player 2 wins)
fn calculate_win_counts(
    player_1_position: u32,
    player_1_current_score: u32,
    player_2_position: u32,
    player_2_current_score: u32,
    memoization_cache: &mut Vec<Option<(u64, u64)>>,
) -> (u64, u64) {
    let memoization_cache_offset = ((player_1_position as usize & 15) << 14)
        + ((player_1_current_score as usize & 31) << 9)
        + ((player_2_position as usize & 15) << 5)
        + (player_2_current_score as usize & 31);

    match memoization_cache[memoization_cache_offset] {
        None => (),
        Some(result) => return result,
    }
    // println!("Calculating win counts.\nPlayer 1 position: {}\nPlayer 1 current score: {}\nPlayer 2 position: {}\nPlayer 2 current score: {}\n", player_1_position, player_1_current_score, player_2_position, player_2_current_score);
    let dirac_die_result_sets = [
        [1, 1, 1],
        [1, 1, 2],
        [1, 1, 3],
        [1, 2, 1],
        [1, 2, 2],
        [1, 2, 3],
        [1, 3, 1],
        [1, 3, 2],
        [1, 3, 3],
        [2, 1, 1],
        [2, 1, 2],
        [2, 1, 3],
        [2, 2, 1],
        [2, 2, 2],
        [2, 2, 3],
        [2, 3, 1],
        [2, 3, 2],
        [2, 3, 3],
        [3, 1, 1],
        [3, 1, 2],
        [3, 1, 3],
        [3, 2, 1],
        [3, 2, 2],
        [3, 2, 3],
        [3, 3, 1],
        [3, 3, 2],
        [3, 3, 3],
    ];
    let mut player_1_win_count: u64 = 0;
    let mut player_2_win_count: u64 = 0;

    for player_1_dirac_die_result_set in &dirac_die_result_sets {
        let mut new_player_1_position = player_1_position;
        let mut new_player_1_score = player_1_current_score;
        for dirac_die_result in player_1_dirac_die_result_set {
            new_player_1_position = (new_player_1_position + dirac_die_result) % 10;
        }
        if new_player_1_position == 0 {
            new_player_1_position = 10;
        }
        new_player_1_score += new_player_1_position;
        let new_player_1_position = new_player_1_position;
        let new_player_1_score = new_player_1_score;
        if new_player_1_score >= 21 {
            // println!(
            //     "Player 1 won with Dirac die results set {:?}\n",
            //     player_1_dirac_die_result_set
            // );
            player_1_win_count += 1;
            continue;
        }
        // This set of die results does not let player 1 win this round.
        for player_2_dirac_die_result_set in &dirac_die_result_sets {
            let mut new_player_2_position = player_2_position;
            let mut new_player_2_score = player_2_current_score;
            for dirac_die_result in player_2_dirac_die_result_set {
                new_player_2_position += dirac_die_result;
            }
            new_player_2_position %= 10;
            if new_player_2_position == 0 {
                new_player_2_position = 10;
            }
            new_player_2_score += new_player_2_position;
            let new_player_2_position = new_player_2_position;
            let new_player_2_score = new_player_2_score;
            if new_player_2_score >= 21 {
                // println!(
                //     "Player 2 won with Dirac die results set {:?}\n",
                //     player_2_dirac_die_result_set
                // );
                player_2_win_count += 1;
                continue;
            }
            // This set of die results does not let player 2 win this round either.
            // println!("Neither player won this round.\nPlayer 1 Dirac die result set: {:?}, Player 2 Dirac die result set: {:?}\n", player_1_dirac_die_result_set, player_2_dirac_die_result_set);
            let (next_round_player_1_win_count, next_round_player_2_win_count) =
                calculate_win_counts(
                    new_player_1_position,
                    new_player_1_score,
                    new_player_2_position,
                    new_player_2_score,
                    memoization_cache,
                );
            player_1_win_count += next_round_player_1_win_count;
            player_2_win_count += next_round_player_2_win_count;
        }
    }
    memoization_cache[memoization_cache_offset] = Some((player_1_win_count, player_2_win_count));
    (player_1_win_count, player_2_win_count)
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(21)), "711480");
}

#[test]
fn test_part_2() {
    assert_eq!(
        solve_part_2(test_helpers::load_puzzle_input(21)),
        "265845890886828"
    );
}
