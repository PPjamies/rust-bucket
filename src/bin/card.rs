#[derive(Debug)]
struct Card<T> {
    val: T,
}

impl<T> Card<T> {
    fn add(&mut self, value: &T)
    where
        T: std::ops::Add<Output=T> + Copy,
    {
        self.val = self.val + *value
    }

    fn dump(&self)
    where
        T: std::fmt::Debug,
    {
        println!("Card is: {:?}", self)
    }
}


fn main() {
    let mut card = Card { val: 5 };
    card.add(&1);
    card.dump();
}