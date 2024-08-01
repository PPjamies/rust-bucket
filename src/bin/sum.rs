fn get_sum(numbers: &Vec<i32>) -> i32 {
    // sum() consumes the iterator
    numbers.iter().sum()
}

fn incr_nums(numbers: &Vec<i32>, increment: i32) -> Vec<i32> {
    // collect() consumes the iterator
    // iter() returns &T
    numbers.iter().map(|x| x + increment).collect()
}

#[test]
fn sum_test() {
    let numbers = vec![1, 2, 3];

    assert_eq!(6, get_sum(&numbers));
    assert_eq!(
        vec![2, 3, 4],
        incr_nums(&numbers, 1)
    );
}
