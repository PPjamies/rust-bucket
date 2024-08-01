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

#[test]
fn bank_balance_test() {
    let bank_balance = BankBalance { val: 800.45 };
    let other_bank_balance = BankBalance { val: 1001.25 };
    println!("Total bank balance is: {}", bank_balance.add(other_bank_balance));
}