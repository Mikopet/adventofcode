use std::fs::File;
use std::io::BufReader;
use std::process;

mod one;
mod shared;

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

    println!("{:?}", one::one(draws, matrices));
    /*
    (&mut &file).rewind().ok();

    match two(&mut reader) {
        Err(e) => println!("{:?}", e),
        Ok(n) => println!("{:?}", n),
    }*/
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_one() {
        let cursor = io::Cursor::new(include_str!("../tests/data/test_input.txt"));
        let (draws, matrices) = shared::file::read(cursor).unwrap();
        assert_eq!(one::one(draws, matrices), 24 * 188);
    }
    /*
    #[test]
    fn test_two() -> std::io::Result<()> {
        let file = File::open("tests/data/test_input.txt")?;
        let reader = BufReader::new(file);
        assert_eq!(one(reader).unwrap(), 13 * 148);
        Ok(())
    }*/
}
