#[derive(Debug)]
struct BingoSquare {
    number: u32,
    called: bool,
}

#[derive(Debug)]
struct BingoBoard {
    //each inner vector is a row
    squares: Vec<Vec<BingoSquare>>,
}

impl BingoBoard {
    fn mark_number(&mut self, called_number: &u32) {
        for x in 0..5 {
            for y in 0..5 {
                if self.squares[x][y].number == *called_number {
                    self.squares[x][y].called = true;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        for row in &self.squares {
            if row
                .iter()
                .map(|square| square.called)
                .reduce(|called_1, called_2| called_1 && called_2)
                .unwrap()
            {
                return true;
            }
        }
        for column_number in 0..5 {
            if self
                .squares
                .iter()
                .map(|row| row[column_number].called)
                .reduce(|called_1, called_2| called_1 && called_2)
                .unwrap()
            {
                return true;
            }
        }
        false
    }

    fn sum_of_unmarked_squares(&self) -> u32 {
        self.squares
            .iter()
            .map(|row| -> u32 {
                row.iter()
                    .filter(|square| !square.called)
                    .map(|square| -> u32 { square.number })
                    .sum()
            })
            .sum()
    }
}

pub(crate) fn solve_part_1(puzzle_input: String) -> u32 {
    let mut section_iterator = puzzle_input.split("\n\n");

    let numbers_called: Vec<u32> = section_iterator
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = Vec::new();

    for board_section in section_iterator {
        let new_squares = board_section
            .split("\n")
            .filter(|line| line.len() != 0)
            .map(|line| -> Vec<BingoSquare> {
                line.split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .map(|num| BingoSquare {
                        number: num,
                        called: false,
                    })
                    .collect()
            })
            .collect::<Vec<Vec<BingoSquare>>>();
        boards.push(BingoBoard {
            squares: new_squares,
        });
    }

    for number_called in numbers_called.iter() {
        for board in &mut boards {
            board.mark_number(number_called);
            if board.is_winner() {
                return board.sum_of_unmarked_squares() * number_called;
            }
        }
    }
    panic!("Failed to find winning board.");
}

pub(crate) fn solve_part_2(puzzle_input: String) -> u32 {
    let mut section_iterator = puzzle_input.split("\n\n");

    let numbers_called: Vec<u32> = section_iterator
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = Vec::new();

    for board_section in section_iterator {
        let new_squares = board_section
            .split("\n")
            .filter(|line| line.len() != 0)
            .map(|line| -> Vec<BingoSquare> {
                line.split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .map(|num| BingoSquare {
                        number: num,
                        called: false,
                    })
                    .collect()
            })
            .collect::<Vec<Vec<BingoSquare>>>();
        boards.push(BingoBoard {
            squares: new_squares,
        });
    }

    for number_called in numbers_called.iter() {
        for board in &mut boards {
            board.mark_number(number_called);
        }
        let non_winning_boards_left: u32 = boards
            .iter()
            .map(|board| {
                if !board.is_winner() {
                    return 1;
                }
                0
            })
            .sum();
        println!("non-winning boards left: {}", non_winning_boards_left);
        if non_winning_boards_left > 0 {
            boards = boards
                .into_iter()
                .filter(|board| !board.is_winner())
                .collect();
            continue;
        }
        return boards.last().unwrap().sum_of_unmarked_squares() * number_called;
    }
    panic!("Failed to find last winning board.");
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(4)), 55770);
}

#[test]
fn test_part_2() {
    assert_eq!(solve_part_2(test_helpers::load_puzzle_input(4)), 2980);
}
