pub struct CandyBag {
    pub name: String,
    pub curr_quantity: i64,
    pub incr: i64,
}

pub fn create_candy_bag(c_name: String, c_curr_quantity: i64, c_incr: i64) -> CandyBag {
    CandyBag {
        name: c_name,
        curr_quantity: c_curr_quantity,
        incr: c_incr,
    }
}

impl Iterator for CandyBag {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.curr_quantity;
        if res <= 0 {
            None
        } else {
            self.curr_quantity -= self.incr;
            Some(res)
        }
    }
}