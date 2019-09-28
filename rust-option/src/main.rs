fn main() {
    let test = Some(1);
    test.map(|v| v+1);

    println!("{:?}", test);

}
