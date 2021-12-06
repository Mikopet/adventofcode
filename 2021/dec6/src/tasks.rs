use crate::shared::*;
use pool::Pool;

pub fn task(pool: Pool, times: u16) -> u64 {
    let pool: Pool = pool.age(times);
    pool.get().len() as u64
}
