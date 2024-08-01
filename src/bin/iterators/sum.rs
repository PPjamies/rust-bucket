fn itr_sum(numbers: &Vec<i32>) {
    numbers.iter().sum(); // consumes the iterator
}


#[test]
fn itr_sum_test() {
    let numbers = vec![1, 2, 3];
    assert_eq!(6, itr_sum(numbers));
}

fn main() {}