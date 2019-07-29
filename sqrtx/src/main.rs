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

        thread::sleep(time::Duration::from_millis(1000));
        if square < x {
            println!(
                " <  square: {}, middle: {}, left: {}, right: {} ",
                square, middle, left, right
            );
            left = middle;
        } else {
            println!(
                " >  square: {}, middle: {}, left: {}, right: {} ",
                square, middle, left, right
            );
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
    let s = my_sqrt(2147395599);
    println!("{}", s);
}
