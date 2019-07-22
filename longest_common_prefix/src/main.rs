// 输入: ["flower","flow","flight"]
// 输出: "fl"

// 输入: ["dog","racecar","car"]
// 输出: ""

use std::collections::HashMap;

fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut count = 0;
    let mut hash = HashMap::<String, u32>::new();
    loop {
        let current = "";
        for i in &strs {
            let char = i.truncate(1);
            if current == char {

            }
            if (hash.contains_key(&char)) {

            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
