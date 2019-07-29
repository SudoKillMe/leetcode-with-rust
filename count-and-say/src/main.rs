trait CharAt {
    fn char_at(&self, index: usize) -> Option<char>;
}

impl CharAt for String {
    fn char_at(&self, index: usize) -> Option<char> {
        if let Some(s) = self.get(index..index + 1) {
            return s.chars().next();
        }
        None
    }
}

// "1234" => "11121314"
// "11233" => "211223"
fn count(s: String) -> String {
    let mut res: Vec<char> = vec![];
    let mut i = 0;
    let mut j = 0;
    let mut count = 0;
    while j <= s.len() {
        if s.char_at(i) == s.char_at(j) {
            count += 1;
            j += 1;
        } else {
            res.push(count.to_string().char_at(0).unwrap());
            res.push(s.char_at(i).unwrap());
            i = j;
            count = 0;
        }
    }

    res.iter().collect()

    // "".to_string()
}

fn count_and_say(n: i32) -> String {
    let mut result = "1".to_string();

    if n == 1 {
        return "1".to_string();
    }
    if n == 2 {
        return "11".to_string();
    }

    let mut res = "11".to_string();
    for i in 3..=n {
        res = count(res);
    }

    res

    // let mut current_result = result.clone();
    // let mut current_count = 0;
    // let mut current_char = ' ';
    // for i in 1..=n {
    //     let mut index = 0;
    //     while let Some(char) = result.char_at(index) {
    //         println!(
    //             " current i: {}, current_index: {}, currentChar: {},  char: {},  current_count: {} ",
    //             i, index, current_char, char, current_count
    //         );
    //         if char != current_char {
    //             if current_count != 0 {
    //                 current_result.push_str(&current_count.to_string());
    //             }
    //             if current_char != ' ' {
    //                 current_result.push(current_char);
    //             }
    //             current_char = char;
    //         } else {
    //             current_count += 1;
    //         }
    //         index += 1;
    //     }
    //     // println!(" current i: {}, current_Result: {} ", i, current_result);
    //     // result = current_result;
    // }
}

fn main() {
    // let string = "12345".to_string();
    // let char = string.char_at(5);
    // let result = count_and_say(5);
    let result = count_and_say(5);
    println!("{:?}", result);
}
