use std::fs;
use std::path;

pub(crate) fn load_puzzle_input(day: u8) -> String {
    let mut puzzle_input_path = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    puzzle_input_path.push(format!("input/{}.txt", day));
    fs::read_to_string(puzzle_input_path).unwrap()
}
