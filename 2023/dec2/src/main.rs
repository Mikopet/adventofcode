use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;
use onig::*;

macro_rules! cube_enum {
    ($($ident:ident),+) => {
        #[derive(Debug)]
        enum Cube {
            $($ident(u8),)+
            None(u8)
        }

        impl From<&str> for Cube {
            fn from(s: &str) -> Cube {
                let mut cubes_iter = s.split(' ');
                let count = cubes_iter.next().unwrap().parse::<u8>().ok().unwrap();
                let mut c = cubes_iter.next().unwrap().chars();
                let color = match c.next() {
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    None => String::new(),
                };

                match color.as_ref() {
                    $(stringify!($ident) => Cube::$ident(count),)+
                    _ => Cube::None(0)
                }
            }
        }
    }
}

cube_enum! {Red, Green, Blue}

fn one<R: BufRead>(reader: R) -> io::Result<usize> {
    let mut possible_games: Vec<usize> = vec![];
    let re = Regex::new(r"(\d+:|\d+ \w+)").unwrap();

    for line in reader.lines() {
        let row = line?.clone();
        let mut current_game: usize = 0;
        let mut possible: bool = false;

        for (i, caps) in re.captures_iter(&row).enumerate() {
            if i == 0 {
                current_game = caps.at(1).unwrap().trim_matches(':').parse::<usize>().ok().unwrap();
                possible = true;
                continue;
            }

            match Cube::from(caps.at(1).unwrap()) {
                Cube::Red(i) if i <= 12 => {}
                Cube::Green(i) if i <= 13 => {}
                Cube::Blue(i) if i <= 14 => {}
                _ => {
                    possible = false;
                    break;
                }
            }
        }

        if possible { possible_games.push(current_game); }
    }

    Ok(possible_games.iter().sum())
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
    fn test_one() {
        let cursor = io::Cursor::new(b"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(one(cursor).unwrap(), 8);
    }
}
