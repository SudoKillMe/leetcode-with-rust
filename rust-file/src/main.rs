use std::fs;
use std::env;
fn read () -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        println!("{:?}", dir.file_type());
    }
    Ok(())
}

fn main() {
//    read();
    let mut dir = env::temp_dir();
    println!("{:?}", dir);
}

