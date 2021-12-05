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
    let segments = shared::file::read(reader).unwrap();

    println!("{:?}", one::one(&segments));
    println!("{:?}", two::two(&segments));
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::vents::Segment;
    use std::io;

    fn parse_file() -> Vec<Segment> {
        let cursor = io::Cursor::new(include_str!("../tests/data/test_input.txt"));
        shared::file::read(cursor).unwrap()
    }

    #[test]
    fn test_one() {
        let segments = parse_file();
        assert_eq!(one::one(&segments), 5);
    }

    #[test]
    fn test_two() {
        let segments = parse_file();
        assert_eq!(two::two(&segments), 12);
    }
}
