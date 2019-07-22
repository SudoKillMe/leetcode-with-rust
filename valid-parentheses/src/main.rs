// 入栈出栈
fn is_valid (s: String) -> bool {
    let mut result: Vec<char> = vec![];
    for char in s.chars() {
        // if char == '(' || char == '[' || char == '{' {
        //     result.push(char);
        // }

        // if char == ')' || char == ']' || char == '}' {
        //     result.pop();
        // }
        match char {
            | '(' | '[' | '{' => result.push(char),
            ')' => {
                if result[result.len() - 1] == '(' {
                    result.pop();
                } else {
                    result.push(char);
                }
            },
            ']' => {
                if result[result.len() - 1] == '[' {
                    result.pop();
                } else {
                    result.push(char);
                }
            },
            '}' => {
                if result[result.len() - 1] == '{' {
                    result.pop();
                } else {
                    result.push(char);
                }
            },
            _ => ()
            // | ')' | ']' | '}' => result.pop(),
        }
        // println!("char {}", char);
    }

    result.len() == 0
}

#[test]
fn test_case () {
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
