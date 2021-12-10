mod shared;

use std::collections::{HashMap, HashSet};
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

fn task_two_map_digits(digits: &Vec<String>) -> HashMap<String, char> {
    let mut m: HashMap<String, char> = HashMap::from([
        (digits[0].clone(), '1'),
        (digits[1].clone(), '7'),
        (digits[2].clone(), '4'),
        (digits[9].clone(), '8'),
    ]);

    let one: HashSet<char> = digits[0].chars().collect();
    let four: HashSet<char> = digits[2].chars().collect();
    let bd: HashSet<char> = four.difference(&one).copied().collect();

    for i in digits[3..=5].iter() {
        let q: HashSet<char> = i.chars().collect();
        let diff: HashSet<char> = bd.difference(&q).copied().collect();

        if diff.len() == 0 {
            m.insert(i.clone(), '5');
        } else {
            let diff: HashSet<char> = four.difference(&q).copied().collect();
            if diff.len() == 2 {
                m.insert(i.clone(), '2');
            } else {
                m.insert(i.clone(), '3');
            }
        }
    }

    for i in digits[6..=8].iter() {
        let q: HashSet<char> = i.chars().collect();
        let diff: HashSet<char> = four.difference(&q).copied().collect();

        if diff.len() == 0 {
            m.insert(i.clone(), '9');
        } else {
            let diff: HashSet<char> = bd.difference(&q).copied().collect();
            if diff.len() == 0 {
                m.insert(i.clone(), '6');
            } else {
                m.insert(i.clone(), '0');
            }
        }
    }

    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
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

    #[test]
    fn test_two_map_digits() {
        let default: Vec<&str> = vec![
            "abcefg",  // 0
            "cf",      // 1
            "acdeg",   // 2
            "acdfg",   // 3
            "bcdf",    // 4
            "abdfg",   // 5
            "abdefg",  // 6
            "acf",     // 7
            "abcdefg", // 8
            "abcdfg",  // 9
        ];
        let mut result = HashMap::new();
        for (k, i) in default.iter().enumerate() {
            let c = char::from_digit(k as u32, 10).unwrap();
            result.insert(i.to_string(), c);
        }

        let mut default: Vec<String> = default.iter().map(|s| s.to_string()).collect();
        default.sort_by(|a, b| a.len().cmp(&b.len()));
        assert_eq!(task_two_map_digits(&default), result);
    }
}
