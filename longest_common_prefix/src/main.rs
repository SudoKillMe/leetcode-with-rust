fn longest_common_prefix (strs: Vec<String>) -> String {
    let first = &strs[0];
    let chars = first.chars();
    for (index, some_char) in chars.enumerate() {
        let char = some_char;

        for str in &strs {
            let str_char = str.get(index..index+1).unwrap().chars().next().unwrap();
            println!(" current str: {}, current char: {} ", str, str_char);
            if str_char != char {
                return first.get(0..index).unwrap().to_string();
            }
        }
    }

    first.to_string()

}

#[test]
fn test_common () {
    let vec = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
    assert_eq!(longest_common_prefix(vec), "fl")
}

fn main() {
    println!("Hello, world!");
}
