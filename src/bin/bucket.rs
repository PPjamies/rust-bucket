#[derive(Debug)]
struct Bucket<T> {
    val: T,
}

impl<T> Bucket<T> {
    fn is_same(&self, other: Bucket<T>) -> bool
    where
        T: PartialEq,
    {
        self.val == other.val
    }
}

fn main() {
    let bucket = Bucket { val: 12 };
    let mut is_same = bucket.is_same(Bucket { val: 12 });
    println!("are buckets the same: {}", is_same);

    is_same = bucket.is_same(Bucket { val: 14 });
    println!("are buckets the same: {}", is_same);
}