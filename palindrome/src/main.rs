fn palindrome(x: i32) -> bool {
    if x.is_negative() {
        return false;
    }

    let string = x.to_string();
    let char_array: Vec<char> = string.chars().collect();
    let len = char_array.len();
    let mut i = 0;
    let mut j = len - 1;
    while i != j && i < j {
        if char_array.get(i) != char_array.get(j) {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

fn palindrome_int(x: i32) -> bool {
    if x.is_negative() {
        return false;
    }

    let mut num = x;
    let mut ans = 0;

    while num != 0 {
        let pop = num % 10;
        ans = ans * 10 + pop;

        num /= 10;
    }

    x == ans
}

#[test]
fn test_normal() {
    assert_eq!(true, palindrome_int(121));
    assert_eq!(true, palindrome_int(11));
    assert_eq!(false, palindrome_int(10));
    assert_eq!(false, palindrome_int(100));
}
#[test]
fn test_negative() {
    assert_eq!(false, palindrome_int(-121));
}

fn main() {
    println!("Hello, world!");
}
