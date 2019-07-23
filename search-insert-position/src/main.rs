fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    if nums[nums.len() - 1] < target {
        return nums.len() as i32;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let middle = (left + right) >> 1;

        if nums[middle] == target {
            return middle as i32;
        }

        if target > nums[middle] {
            left = middle + 1;
        } else {
            right = middle;
        }
    }

    left as i32
}

#[test]
fn test_one() {
    let vec = vec![1, 3, 5, 6];
    let target = 5;
    assert_eq!(search_insert(vec, target), 2);
}
#[test]
fn test_two() {
    let vec = vec![1, 3, 5, 6];
    let target = 2;
    assert_eq!(search_insert(vec, target), 1);
}
#[test]
fn test_three() {
    let vec = vec![1, 3, 5, 6];
    let target = 7;
    assert_eq!(search_insert(vec, target), 4);
}
#[test]
fn test_four() {
    let vec = vec![1, 3, 5, 6];
    let target = 0;
    assert_eq!(search_insert(vec, target), 0);
}

fn main() {
    println!("Hello, world!");
}
