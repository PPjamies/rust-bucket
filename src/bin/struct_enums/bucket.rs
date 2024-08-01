#[derive(Debug)]
pub struct Bucket<T> {
    pub val: T,
}

impl<T> Bucket<T> {
    pub fn is_same(&self, other: Bucket<T>) -> bool
    where
        T: PartialEq,
    {
        self.val == other.val
    }
}