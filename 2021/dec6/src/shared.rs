pub mod pool {
    pub struct Fish {
        timer: u8,
    }
    pub struct Pool {
        fishes: Vec<Fish>,
    }

    impl Fish {
        pub fn new(timer: u8) -> Self {
            Fish { timer: timer }
        }

        pub fn age(&self) -> Vec<Fish> {
            let mut vec: Vec<Fish> = vec![];

            if self.timer > 0 {
                vec.push(Fish::new(self.timer - 1))
            } else {
                vec.push(Fish::new(6));
                vec.push(Fish::new(8));
            }

            vec
        }
    }

    impl Pool {
        pub fn from(mut s: String) -> Pool {
            let mut v: Vec<Fish> = vec![];

            if s.ends_with('\n') {
                s.pop();
            }

            for timer in s.split(",") {
                v.push(Fish::new(timer.parse::<u8>().unwrap()))
            }

            Pool { fishes: v }
        }

        pub fn age(self, times: u16) -> Pool {
            let mut sea: Vec<Fish> = self.fishes;

            for i in 0..times {
                let mut pool: Vec<Fish> = vec![];

                for fish in &sea {
                    pool.append(&mut fish.age());
                }

                println!("{:?}: {:?}", i, &pool.len());
                sea = pool;
            }

            Pool { fishes: sea }
        }

        pub fn get(self) -> Vec<Fish> {
            self.fishes
        }
    }
}
