
fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = digits.clone();

    for i in (0..result.len()).rev() {
        let mut num = result[i];

        num += 1;
        num %= 10;

        result[i] = num;

        if num != 0 {
            return result;
        }
    }
    result.insert(0, 1);
    result
}
：q
#[test]
fn test_one() {
    let vec = vec![1,2,3];
    assert_eq!(plus_one(vec), [1,2,4]);
}

#[test]
fn test_two() {
    let vec = vec![4,3,2,1];
    assert_eq!(plus_one(vec), [4,3,2,2]);
}

#[test]
fn test_three() {
    let vec = vec![9];
    assert_eq!(plus_one(vec), [1,0]);
}

fn main() {
    let vec = vec![1,2,3];
    plus_one(vec);
}
