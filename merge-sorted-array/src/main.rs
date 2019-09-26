
fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut nums1_copy = nums1.clone();

    let mut i = 0usize;
    let mut j = 0usize;

    let mut p = 0usize;

    while i < (m as usize) && j < (n as usize) {
        println!("i: {}, j: {}, p: {},  nums1: {:?}, nums1_copy: {:?}, nums2: {:?}", i, j, p,
                 nums1, nums1_copy, nums2);
        if nums1_copy[i] < nums2[j] {
            nums1[p] = nums1_copy[i];
            i += 1;
        } else {
            nums1[p] = nums2[j];
            j += 1;
        }
        p += 1;
    }

}

//#[test]
fn test () {
    let mut vec1 = vec![1,2,3,0,0,0];
    let mut vec2 = vec![2,5,6];
    let m = 3;
    let n = 3;

    merge(&mut vec1, m, &mut vec2, n);

//    assert_eq!(vec1, vec![1,2,2,3,5,6])
}

fn main() {
//    println!("Hello, world!");
    test();
}
