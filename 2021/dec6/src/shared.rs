pub mod pool {
    pub struct Fish(u8);
    pub struct Pool(Vec<Fish>);

    impl Fish {
        pub fn new(timer: u8) -> Self {
            Fish(timer)
        }
    }

    impl Pool {
        pub fn from(mut s: String) -> Self {
            let mut v: Vec<Fish> = vec![];

            if s.ends_with('\n') {
                s.pop();
            }

            for timer in s.split(",") {
                v.push(Fish::new(timer.parse::<u8>().unwrap()))
            }
            Pool(v)
        }
    }
}
