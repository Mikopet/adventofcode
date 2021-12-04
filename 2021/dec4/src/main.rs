use std::fs::File;
use std::io::BufReader;
use std::process;

mod one;
mod shared;
mod two;

fn main() {
    let file = match File::open("input.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("File open error! {}", e);
            process::abort();
        }
    };

    let reader = BufReader::new(&file);
    let (draws, matrices) = shared::file::read(reader).unwrap();

    println!("{:?}", one::one(&draws, &matrices));
    println!("{:?}", two::two(&draws, &matrices));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    fn parse_file() -> (Vec<u8>, Vec<Vec<u8>>) {
        let cursor = io::Cursor::new(include_str!("../tests/data/test_input.txt"));
        shared::file::read(cursor).unwrap()
    }

    #[test]
    fn test_one() {
        let (draws, matrices) = parse_file();
        assert_eq!(one::one(&draws, &matrices), 24 * 188);
    }

    #[test]
    fn test_two() {
        let (draws, matrices) = parse_file();
        assert_eq!(two::two(&draws, &matrices), 13 * 148);
    }
}
