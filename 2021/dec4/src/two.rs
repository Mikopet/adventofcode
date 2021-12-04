use crate::shared::*;

pub fn two(draws: &Vec<u8>, matrices: &Vec<Vec<u8>>) -> u32 {
    let (matrix_index, draws_index) = matrix::most_bingo(&draws, &matrices);
    let unmarked_sum =
        matrix::calculate_unmarked(&matrices[matrix_index], &draws[..draws_index + 1]);
    let loser_number = draws[draws_index] as u32;

    unmarked_sum * loser_number
}
