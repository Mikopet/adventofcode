use crate::shared::*;
use pool::Pool;

pub fn one(pool: Pool) -> u32 {
    let pool: Pool = pool.age(80);
    pool.get().len() as u32
}
