use serde_json::{Result, Value, json};

fn untyped_example() -> Result<()> {
    let data = r#"
        {
            "name": "data",
            "age": 43,
            "phones": [
                "+86 1111222",
                "+86 2222111"
            ]
        }
    "#;

    let v: Value = serde_json::from_str(data)?;

    println!("call {} at number {}", v["nama"], v["phones"][0]);


    let d = json!({
        "name": "dddd",
        "age": 12,
        "phones": [
            "12345",
            "67890"
        ]
    });
    println!("{}", d.to_string());
    Ok(())
}

fn main() {
    untyped_example();
}

