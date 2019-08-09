
fn length_of_last_word(s: String) -> i32 {
    let vec = s.trim().split(char::is_whitespace).collect::<Vec<&str>>();
    let last = vec[vec.len() - 1];
    println!("{:?}", vec);
    if last.len() != 0 {
        last.len() as i32
    } else {
        0 as i32
    }
}

#[test]
fn test(){
    let s = "Hello World".to_string();
    assert_eq!(length_of_last_word(s), 5);
}

fn main() {
    let s = "a ".to_string();
    let result = length_of_last_word(s);
//    println!("{}", result);
}
