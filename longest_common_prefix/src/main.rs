fn longest_common_prefix (strs: Vec<String>) -> String {

    if strs.len() == 0 {
        return "".to_string();
    }

    let first = &strs[0];
    let chars = first.chars();
    for (index, some_char) in chars.enumerate() {
        let char = some_char;

        for str in &strs {
            // let str_char = str.get(index..index+1).unwrap().chars().next().unwrap();
            // if str_char != char {
            //     return first.get(0..index).unwrap().to_string();
            // }


            let str_char = match str.get(index..index+1) {
                Some(val) => val.chars().next().unwrap(),
                None => ' '
            };
            if str_char == ' ' || str_char != char {
                return first.get(0..index).map(|s| s.to_string()).unwrap();
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

#[test]
fn test_none () {
    let vec = vec!["dog".to_string(),"racecar".to_string(),"car".to_string()];
    assert_eq!(longest_common_prefix(vec), "")
}

#[test]
fn test_empty () {
    let vec = vec![];
    assert_eq!(longest_common_prefix(vec), "");
}

fn main() {
    println!("Hello, world!");
}
