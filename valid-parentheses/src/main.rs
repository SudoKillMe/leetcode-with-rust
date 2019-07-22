use std::collections::HashMap;
// 入栈出栈
fn is_valid(s: String) -> bool {
    let mut hash = HashMap::new();
    hash.insert(')', '(');
    hash.insert(']', '[');
    hash.insert('}', '{');
    let mut result: Vec<char> = vec![];
    for char in s.chars() {
        match char {
            '(' | '[' | '{' => result.push(char),
            val => {
                if hash.contains_key(&val) {
                    // 闭合符号
                    let top = match result.len() {
                        0 => '#',
                        _ => result.pop().unwrap(),
                    };
                    if top != *hash.get(&val).unwrap() {
                        return false;
                    }
                } else {
                    result.push(char);
                }
            }
            // ')' => {
            //     if result.len() == 0 {
            //         return false;
            //     }
            //     if result[result.len() - 1] == '(' {
            //         result.pop();
            //     } else {
            //         result.push(char);
            //     }
            // }
            // ']' => {
            //     if result.len() == 0 {
            //         return false;
            //     }
            //     if result[result.len() - 1] == '[' {
            //         result.pop();
            //     } else {
            //         result.push(char);
            //     }
            // }
            // '}' => {
            //     if result.len() == 0 {
            //         return false;
            //     }
            //     if result[result.len() - 1] == '{' {
            //         result.pop();
            //     } else {
            //         result.push(char);
            //     }
            // }
            _ => (), // | ')' | ']' | '}' => result.pop(),
        }
        // println!("char {}", char);
    }

    result.len() == 0
}

#[test]
fn test_case() {
    let str1 = "()".to_string();
    let str2 = "()[]{}".to_string();

    assert_eq!(is_valid(str1), true);
    assert_eq!(is_valid(str2), true);

    let str_false_1 = "(]".to_string();
    let str_false_2 = "(])[".to_string();

    assert_eq!(is_valid(str_false_1), false);
    assert_eq!(is_valid(str_false_2), false);

    let str_nest = "{[]}".to_string();

    assert_eq!(is_valid(str_nest), true);
}

fn main() {
    let test = "()[]".to_string();
    is_valid(test);
    // println!("Hello, world!");
}
