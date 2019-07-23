fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut i = 0;
    while i < nums.len() {
        if nums[i] == val {
            nums.remove(i);
        } else {
            i += 1;
        }
    }

    nums.len() as i32
}

#[test]
fn test_normal() {
    let mut vec = vec![3, 2, 2, 3];
    assert_eq!(remove_element(&mut vec, 3), 2);
}
#[test]
fn test_large() {
    let mut vec = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(remove_element(&mut vec, 2), 5);
}

fn main() {
    println!("Hello, world!");
}
