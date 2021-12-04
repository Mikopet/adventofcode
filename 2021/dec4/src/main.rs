use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;

fn one_search_horizontal(boards: Vec<Vec<Vec<u8>>>, numbers: Vec<u8>) -> (usize, u8) {
    let mut board_index: usize = 0;
    let mut winning_number: i8 = -1;

    for (index, board) in boards.iter().enumerate() {
        let mut subset: Vec<u8> = vec![];
        let mut winner: bool = false;

        for n in numbers.clone() {
            subset.push(n);
            if winning_number == n as i8 { break; }

            for row in board {
                if row.iter().filter(|&i| subset.contains(&i)).collect::<Vec<&u8>>().len() == 5 { winner = true; break; }
            }

            if winner {
                winning_number = n as i8;
                break;
            }
        }

        if winner { board_index = index; }
    }

    (board_index, winning_number as u8)
}

fn calculate_unmarked(board: &Vec<Vec<u8>>, numbers: Vec<u8>, winner: u8) -> u32 {
    let index = numbers.iter().position(|&r| r == winner).unwrap();
    let winner_row: &Vec<u8> = &(&numbers[..index+1]).to_vec();
    let board_numbers = board.iter().flatten().collect::<Vec<_>>();

    let unmarked_numbers: Vec<&u8> = board_numbers.iter().filter(|x| !winner_row.contains(x)).cloned().collect();

    unmarked_numbers.iter().fold(0, |acc, x| acc + x.clone().clone() as u32)
}

fn one<R: BufRead>(mut reader: R) -> io::Result<u32> {
    let mut first_line = String::new();
    reader
        .read_line(&mut first_line)
        .expect("Unable to read winner numbers");

    first_line = first_line.replace("\n", "");

    let numbers: Vec<u8> = first_line
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut boards: Vec<Vec<Vec<u8>>> = vec![];
    let mut board: Vec<Vec<u8>> = vec![];

    for line in reader.lines() {
        let row: String = line?.to_string();

        if row.len() == 0 {
            if board.len() == 5 {
                boards.push(board);
            }
            board = vec![];
            continue;
        }

        let vec: Vec<u8> = row
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        board.push(vec);
    }
    boards.push(board);

    let (board_index, winning_number) = one_search_horizontal(boards.clone(), numbers.clone());
    let unmarked_sum = calculate_unmarked(&boards[board_index], numbers, winning_number);

    Ok(winning_number as u32 * unmarked_sum as u32)
}

fn main() {
    let file;

    match File::open("input.txt") {
        Ok(f) => file = f,
        Err(e) => {
            println!("File open error! {}", e);
            process::abort();
        }
    }

    let mut reader = BufReader::new(&file);

    match one(&mut reader) {
        Err(e) => println!("{:?}", e),
        Ok(n) => println!("{:?}", n),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() -> std::io::Result<()> {
        let file = File::open("tests/data/test_input.txt")?;
        let reader = BufReader::new(file);
        assert_eq!(one(reader).unwrap(), 24 * 188);
        Ok(())
    }
}
