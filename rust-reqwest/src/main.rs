extern crate reqwest;

use std::collections::HashMap;
use std::hash::Hash;


fn main() -> Result<(), Box<std::error::Error>> {
    let resp = reqwest::get("https://www.baidu.com").unwrap();
//    println!("text{:#?}", resp.text());
    println!("length{:#?}", resp.content_length());
    println!("headers{:#?}", resp.headers());
    let cookies = resp.cookies();
    for cookie in cookies {
        println!("cookie{:#?}", cookie.name());
    }

    Ok(())
}
