#![no_std]

pub mod sys;

#[test]
fn test_example() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}