use std::{thread, time};

fn my_sqrt(x: i32) -> i32 {
    let mut left = 0;
    let mut right = x;

    while left < right {
        let middle = (left + right + 1) >> 1;
        let square = middle * middle;

        if square == x {
            return middle as i32;
        }

        if square < x {
            left = middle;
        } else {
            right = middle - 1;
        }
    }

    left as i32
}

#[test]
fn test() {
    let x = 4;
    assert_eq!(my_sqrt(x), 2);
}

#[test]
fn test_one() {
    let x = 8;
    assert_eq!(my_sqrt(x), 2);
}

fn main() {
    my_sqrt(8);
}
