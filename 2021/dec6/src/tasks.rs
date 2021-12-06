use crate::shared::*;
use pool::Pool;

pub fn task(pool: Pool, times: u16) -> u128 {
    let pool: Pool = pool.age(times);
    pool.get().iter().sum::<u128>()
}
