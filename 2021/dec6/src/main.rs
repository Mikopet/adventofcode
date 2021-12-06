mod shared;
mod tasks;

use shared::pool::Pool;

fn main() {
    let pool: Pool = Pool::from(include_str!("../input.txt").to_string());

    println!("{:?}", tasks::one(pool));
    //    println!("{:?}", two::two(&segments));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_one() {
        let pool = Pool::from("3,4,3,1,2".to_string());
        assert_eq!(tasks::one(pool), 5934);
    }
}
