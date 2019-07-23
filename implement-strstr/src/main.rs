fn str_str(haystack: String, needle: String) -> i32 {
    let window = needle.len();

    if window == 0 {
        return 0;
    }

    let mut start = 0;
    let mut end = start + window;

    while end <= haystack.len() {
        let slice = haystack.get(start..end).unwrap();
        if slice == needle {
            return start as i32;
        }
        start += 1;
        end += 1;
    }
    -1
}

#[test]
fn test_normal() {
    let haystack = "hello".to_string();
    let needle = "ll".to_string();
    assert_eq!(str_str(haystack, needle), 2);
}
#[test]
fn test_fail() {
    let haystack = "aaaaa".to_string();
    let needle = "bba".to_string();
    assert_eq!(str_str(haystack, needle), -1);
}
#[test]
fn test_zero() {
    let haystack = "aaaaa".to_string();
    let needle = "".to_string();
    assert_eq!(str_str(haystack, needle), 0);
}
#[test]
fn test_short() {
    let haystack = "a".to_string();
    let needle = "a".to_string();
    assert_eq!(str_str(haystack, needle), 0);
}

fn main() {
    println!("Hello, world!");
}
