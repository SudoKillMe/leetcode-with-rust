use std::collections::HashMap;

fn remove_duplicates (nums: &mut Vec<i32>) -> i32 {

    let mut map = HashMap::new();
    let mut length = 0;
    while length < nums.len() {
        let num = nums[length];
        if map.contains_key(&num) {
            nums.remove(length);
            length -= 1;
        } else {
            map.insert(num, 1);
        }
        length += 1;
    }

    nums.len() as i32

}

#[test]
fn test_normal () {
    let mut vec = vec![1,1,2];
    let num = remove_duplicates(&mut vec);
    println!("normal {:?}", vec);
    assert_eq!(num, 2)
}
#[test]
fn test_long () {
    let mut vec = vec![0,0,1,1,1,2,2,3,3,4];
    let num = remove_duplicates(&mut vec);
    println!("long {:?}", vec);
    assert_eq!(num, 5)
}

fn main() {
    let mut vec = vec![0,0,1,1,1,2,2,3,3,4];
    let num = remove_duplicates(&mut vec);
    println!("normal {:?}", vec);
}
