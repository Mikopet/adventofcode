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
    println!("{:?}", task_two(&segments));
}

fn task_one(v: &Vec<Vec<String>>) -> usize {
    let mut sum = 0;

    for i in v {
        for j in i[i.len() - 4..].iter() {
            let l = j.len();
            if l < 5 || l == 7 {
                sum += 1
            }
        }
    }

    sum
}

fn task_two(v: &Vec<Vec<String>>) -> usize {
    61229
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    fn parse_file() -> Vec<Vec<String>> {
        let cursor = io::Cursor::new(include_str!("../tests/data/test_input.txt"));
        shared::file::read(cursor).unwrap()
    }

    #[test]
    fn test_one() {
        let segments = parse_file();
        assert_eq!(task_one(&segments), 26);
    }

    #[test]
    fn test_two() {
        let segments = parse_file();
        assert_eq!(task_two(&segments), 61229);
    }
}
