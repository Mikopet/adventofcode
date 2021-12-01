use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::process;

fn one<R: BufRead>(reader: R) -> io::Result<u16> {
    let mut increment_count: u16 = 0;
    let mut previous_depth: u16 = 0;

    for line in reader.lines() {
        let depth: u16 = line?.to_string().parse::<u16>().unwrap();
        if depth > previous_depth {
            increment_count += 1;
        }
        previous_depth = depth;
    }

    Ok(increment_count - 1)
}

fn two<R: BufRead>(reader: R) -> io::Result<u16> {
    let mut increment_count: u16 = 0;
    let mut current_depths: Vec<u16> = vec![];

    for line in reader.lines() {
        let previous_depths = current_depths.clone();

        let depth: u16 = line?.to_string().parse::<u16>().unwrap();
        current_depths.push(depth);

        if previous_depths.len() < 3 {
            continue;
        } else {
            current_depths.remove(0);
        }

        let previous_sum: u16 = previous_depths.iter().sum();
        let current_sum: u16 = current_depths.iter().sum();

        if current_sum > previous_sum {
            increment_count += 1;
        }
    }

    Ok(increment_count)
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
