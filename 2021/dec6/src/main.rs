mod shared;
mod tasks;

use shared::pool::Pool;
use std::env;

fn main() {
    let pool: Pool = Pool::from(include_str!("../input.txt").to_string());
    let args: Vec<String> = env::args().collect();
    let times: u16 = args.last().unwrap().parse::<u16>().unwrap();

    println!("{:?}", tasks::task(pool, times));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let pool = Pool::from("3,4,3,1,2".to_string());
        assert_eq!(tasks::task(pool, 80), 5934);
    }
}
