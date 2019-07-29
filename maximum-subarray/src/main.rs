
fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }


    let mut sum = 0;
    let mut ans = nums[0];

    for num in nums {
        if sum > 0 {
           sum += num;
        }  else {
           sum = num;
        }

        ans = std::cmp::max(ans, sum);

    }

    ans
}

#[test]
fn test_normal () {
    let vec = vec![-2,1,-3,4,-1,2,1,-5,4];
    assert_eq!(max_sub_array(vec), 6);
}

fn main() {
    let vec = vec![-2,1,-3,4,-1,2,1,-5,4];
    max_sub_array(vec);
}
