pub mod pool {
    pub struct Pool {
        fishes: [u128; 9],
    }

    impl Pool {
        pub fn from(mut s: String) -> Pool {
            let mut a: [u128; 9] = [0; 9];

            if s.ends_with('\n') {
                s.pop();
            }

            for timer in s.split(",") {
                a[timer.parse::<usize>().unwrap()] += 1;
            }

            Pool { fishes: a }
        }

        // unfortunately not my own solution. I was influenced...
        pub fn age(self, times: u16) -> Pool {
            let mut a: [u128; 9] = self.fishes;

            for _ in 0..times {
                a = [a[1], a[2], a[3], a[4], a[5], a[6], a[7] + a[0], a[8], a[0]]
            }

            Pool { fishes: a }
        }

        pub fn get(self) -> [u128; 9] {
            self.fishes
        }
    }
}
