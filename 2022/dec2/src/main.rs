use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;

fn one<R: BufRead>(reader: R) -> io::Result<usize> {
    let mut points: usize = 0;

    for line in reader.lines() {
        let s: String = line?.to_string();
        let o = s.chars().next().unwrap(); // opponent

        match s.chars().last().unwrap() {
            'X' => { // Rock
                points += 1;
                if o == 'A' { points += 3 } // Rock
                if o == 'C' { points += 6 } // Scissors
            },
            'Y' => { // Paper
                points += 2;
                if o == 'A' { points += 6 } // Rock
                if o == 'B' { points += 3 } // Paper
            },
            'Z' => { // Scissors
                points += 3;
                if o == 'B' { points += 6 } // Paper
                if o == 'C' { points += 3 } // Scissors
            },
            _ => {}
        }
    }

    Ok(points)
}

fn two<R: BufRead>(reader: R) -> io::Result<usize> {
    let mut points: usize = 0;

    for line in reader.lines() {
        let s: String = line?.to_string();
        let a = s.chars().last().unwrap(); // act

        match s.chars().next().unwrap() {
            'A' => { // Rock
                if a == 'X' { points += 3 } // Lose (3+0)
                if a == 'Y' { points += 4 } // Draw (1+3)
                if a == 'Z' { points += 8 } // Win (2+6)
            },
            'B' => { // Paper
                if a == 'X' { points += 1 } // Lose (1+0)
                if a == 'Y' { points += 5 } // Draw (2+3)
                if a == 'Z' { points += 9 } // Win (3+6)
            },
            'C' => { // Scissors
                if a == 'X' { points += 2 } // Lose (2+0)
                if a == 'Y' { points += 6 } // Draw (3+3)
                if a == 'Z' { points += 7 } // Win (1+6)
            },
            _ => {}
        }
    }

    Ok(points)
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

    (&mut &file).rewind().ok();

    match two(&mut reader) {
        Err(e) => println!("{:?}", e),
        Ok(n) => println!("{:?}", n),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let cursor = io::Cursor::new(b"A Y\nB X\nC Z");
        assert_eq!(one(cursor).unwrap(), 15);
    }

    #[test]
    fn test_two() {
        let cursor = io::Cursor::new(b"A Y\nB X\nC Z");
        assert_eq!(two(cursor).unwrap(), 12);
    }
}

