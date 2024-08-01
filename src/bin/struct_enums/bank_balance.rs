use std::ops::Add;

#[derive(Debug)]
pub struct BankBalance {
    pub val: f64,
}

impl Add for BankBalance {
    type Output = f64;

    fn add(self, rhs: Self) -> Self::Output {
        self.val + rhs.val
    }
}