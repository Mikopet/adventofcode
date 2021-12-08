mod shared;

use std::fs::File;
use std::io::BufReader;
use std::process;

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

    println!("{:?}", task_one(&segments));
//    println!("{:?}", task_two(&segments));
}

fn task_one(v: &Vec<usize>) -> usize {
    v.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    fn parse_file() -> Vec<usize> {
        let cursor = io::Cursor::new(include_str!("../tests/data/test_input.txt"));
        shared::file::read(cursor).unwrap()
    }

    #[test]
    fn test_one() {
        let segments = parse_file();
        assert_eq!(task_one(&segments), 26);
    }
}
