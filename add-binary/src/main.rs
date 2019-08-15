
fn add_binary(a: String, b: String) -> String {
    let mut a_vec = a.chars().collect::<Vec<char>>();
    let mut b_vec = b.chars().collect::<Vec<char>>();

    while a_vec.len() > b_vec.len() {
        b_vec.insert(0, '0');
    }

    while a_vec.len() < b_vec.len() {
        a_vec.insert(0, '0');
    }

    let mut result: Vec<u32> = vec![];

    let mut carry = 0;
    for i in (0..a_vec.len()).rev() {
        let mut num = 0;
        let a_num = a_vec[i].to_digit(10);
        let b_num = b_vec[i].to_digit(10);

        num += (a_num.unwrap() + b_num.unwrap() + carry);
        carry = num / 2;
        num %= 2;

        result.insert(0, num);
    }

    if carry != 0 {
        result.insert(0, carry);
    }

    result.iter().map(|i| i.to_string()).collect::<String>()

}


#[test]
fn test_normal() {
    let a = String::from("10101");
    let b = String::from("10000");

    assert_eq!(add_binary(a, b), String::from("100101"));
}

#[test]
fn test_one() {
    let a = String::from("11");
    let b = String::from("1");

    assert_eq!(add_binary(a, b), String::from("100"));
}


#[test]
fn test_two() {
    let a = String::from("1010");
    let b = String::from("1011");

    assert_eq!(add_binary(a, b), String::from("10101"));
}

fn main() {



    let chcar = 'a';
    let test: Vec<char> = String::from("ddddsss").chars().collect();
    println!("{:?}", test)

}
