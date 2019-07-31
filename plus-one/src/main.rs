
fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 0;
    let mut result: Vec<i32> = vec![];

    for i in (0..digits.len()).rev() {
        let num = digits[i];
        let mut t = num + carry;
        if i == digits.len() - 1 {
            t += 1;
        }
        if t >= 10 {
            carry = t / 10;
            t = t % 10;
        } else {
            carry = 0;
        }
        result.insert(0, t);
    }

    result
}

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

fn main() {
    let vec = vec![1,2,3];
    plus_one(vec);
}
