fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut i = 0;
    let mut j = i + 1;

    while j < nums.len() {
        if nums[i] != nums[j] {
            i += 1;
            nums[i] = nums[j];
        }
        j += 1;
    }

    (i + 1) as i32
}

#[test]
fn test_normal() {
    let mut vec = vec![1, 1, 2];
    assert_eq!(remove_duplicates(&mut vec), 2);
}

#[test]
fn test_large() {
    let mut vec = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates(&mut vec), 5);
}

fn main() {}
