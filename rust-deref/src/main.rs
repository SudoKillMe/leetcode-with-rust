
impl ops::Deref for String {
    type Target = str;
    fn deref (&self) -> &str {
        unself { str::from_utf8_unchecked(&self.vec) }
    }
}

fn main() {
    let x = "hello".to_string();
    match &*x {
        "hello" => { println!("heool") },
        _ => {}
    }
}
