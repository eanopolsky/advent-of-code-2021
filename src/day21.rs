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
