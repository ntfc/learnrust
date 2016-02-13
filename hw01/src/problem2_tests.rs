#![cfg(test)]

use problem2::vec_mult;

#[test]
fn test_vec_mult() {
    let a = vec![1, 2, 3];
    let b = vec![3, 4, 5];
    assert_eq!(vec_mult(&a, 2), 12);
    assert_eq!(vec_mult(&b, 5), 60);
}

