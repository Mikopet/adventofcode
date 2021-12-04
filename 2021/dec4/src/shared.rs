pub mod matrix {
    pub fn calculate_unmarked(matrix: &Vec<u8>, winner_draws: &[u8]) -> u32 {
        let unmarked_numbers: Vec<u8> = matrix
            .iter()
            .filter(|n| !winner_draws.contains(n))
            .cloned()
            .collect();
        unmarked_numbers
            .iter()
            .fold(0, |acc, x| acc + x.clone().clone() as u32)
    }

    pub fn least_bingo(draws: &Vec<u8>, matrices: &Vec<Vec<u8>>) -> (usize, usize) {
        let mut least_position: usize = draws.len();
        let mut matrix_index: usize = 0;

        for (index, matrix) in matrices.iter().enumerate() {
            let original_position = bingo_at(matrix, &draws);
            let mut transposed = vec![0; 25];
            transpose::transpose(&matrix, &mut transposed, 5, 5);
            let transposed_position = bingo_at(&transposed, &draws);

            if original_position < least_position || transposed_position < least_position {
                matrix_index = index;
                least_position = original_position.min(transposed_position);
            }
        }

        (matrix_index, least_position)
    }

    pub fn most_bingo(draws: &Vec<u8>, matrices: &Vec<Vec<u8>>) -> (usize, usize) {
        let mut most_position: usize = 0;
        let mut matrix_index: usize = 0;

        for (index, matrix) in matrices.iter().enumerate() {
            let original_position = bingo_at(matrix, &draws);
            let mut transposed = vec![0; 25];
            transpose::transpose(&matrix, &mut transposed, 5, 5);
            let transposed_position = bingo_at(&transposed, &draws);

            if original_position > most_position && transposed_position > most_position {
                matrix_index = index;
                most_position = original_position.min(transposed_position);
            }
        }

        (matrix_index, most_position)
    }

    fn bingo_at(matrix: &Vec<u8>, draws: &Vec<u8>) -> usize {
        let mut current_draws: Vec<&u8> = vec![];
        let mut bingo_position: usize = draws.len();

        for (draw_index, draw) in draws.iter().enumerate() {
            current_draws.push(draw);

            let mut row: Vec<u8> = vec![];
            let mut bingo: bool = false;
            for n in matrix {
                row.push(n.clone());
                if row.len() == 5 {
                    if row
                        .iter()
                        .filter(|&i| current_draws.contains(&i))
                        .collect::<Vec<&u8>>()
                        .len()
                        == 5
                    {
                        bingo = true;
                        break;
                    }
                    row = vec![];
                }
            }
            if bingo {
                bingo_position = draw_index;
                break;
            }
        }

        bingo_position
    }
}

pub mod file {
    use lazy_regex::*;
    use std::io::{self, prelude::*};

    pub static DIGITS_RE: Lazy<Regex> = lazy_regex!(r"\d+");

    pub fn read<R: BufRead>(mut reader: R) -> io::Result<(Vec<u8>, Vec<Vec<u8>>)> {
        let mut first_line = String::new();
        reader
            .read_line(&mut first_line)
            .expect("Unable to read winner numbers");
        let draw_numbers: Vec<u8> = find_digits(&first_line);

        let mut matrices: Vec<Vec<u8>> = vec![];
        let mut matrix: Vec<u8> = vec![];

        for line in reader.lines() {
            let row: String = line?.to_string();

            if row.len() == 0 {
                if matrix.len() > 0 {
                    matrices.push(matrix)
                }
                matrix = vec![];
                continue;
            }

            for n in find_digits(&row) {
                matrix.push(n);
            }
        }

        if matrix.len() > 0 {
            matrices.push(matrix)
        }

        Ok((draw_numbers, matrices))
    }

    fn find_digits(s: &str) -> Vec<u8> {
        DIGITS_RE
            .find_iter(&s)
            .filter_map(|d| d.as_str().parse::<u8>().ok())
            .collect()
    }
}
