use std::{thread, time};
fn search (nums: Vec<i32>, target: i32) -> i32 {

    if nums.len() == 0 {
        return -1;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let mid = (left + right + 1) >> 1;
        if nums[mid] == target {
            return mid as i32;
        }
        // thread::sleep(time::Duration::from_millis(1000));
        if nums[mid] > target {
            println!("> mid: {}, left: {}, right: {}", mid, left, right);

            right = mid - 1;
        } else {
            println!("< mid: {}, left: {}, right: {}",mid, left, right );
            left = mid;
        }
    }

    if nums[left] == target { left as i32 } else {-1}
}

#[test]
fn test_normal () {
    let vec = vec![-1,0,3,5,9,12];
    let target = 9;
    assert_eq!(search(vec, target), 4);
}
#[test]
fn test_none () {
    let vec = vec![-1,0,3,5,9,12];
    let target = 2;
    assert_eq!(search(vec, target), -1);
}

fn main() {
    let vec = vec![-1,0,3,5,9,12];
    let target = 2;
    search(vec, target);
}
