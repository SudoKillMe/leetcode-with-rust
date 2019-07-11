use std::num::Wrapping;

fn reverse(num: i32) -> i32 {
    let mut num = num;
    let mut ans = 0i32;
    while num != 0 {
        let pop = num % 10;
        // ans = ans * 10 + pop;

        ans = match ans.checked_mul(10) {
            Some(val) => match val.checked_add(pop) {
                Some(v) => v,
                None => return 0,
            },
            None => return 0,
        };

        // ans = ans * 10 + pop;
        // println!("ans: {}, pop: {},  num: {}", ans, pop, num);
        // if ans > std::i32::MAX || ans < std::i32::MIN {
        //     return 0;
        // }

        num /= 10;
    }

    ans
}

#[test]
fn test_normal() {
    assert_eq!(4321, reverse(1234));
}
#[test]
fn test_negative() {
    assert_eq!(-321, reverse(-123));
}
#[test]
fn test_with_zero() {
    assert_eq!(321, reverse(1230));
}
#[test]
fn test_overflow() {
    assert_eq!(0, reverse(1534236469));
}
#[test]
fn test_negative_overflow() {
    assert_eq!(0, reverse(-1534236469));
}

fn main() {
    let n = 15342364699;
    // reverse(1534236469);
    // let a = 15342364699;
    // a as i32;
}
