
//假设你正在爬楼梯，需要 n 阶你才能到达楼顶。
//
//每次你可以爬 1 或 2 个台阶，你有多少种不同的方法可以爬到楼顶呢？

fn climb_stairs(n: i32) -> i32 {
    climb_recurse(0, n)
}

fn climb_recurse(i: i32, n: i32) -> i32 {
    if i > n {
        return 0;
    }
    if i == n {
        return 1;
    }

    climb_recurse(i+1, n) + climb_recurse(i+2, n)
}

fn climb_stairs_dy(n: i32) -> i32 {
    let mut result = vec![0; (n + 1) as usize];

    if n == 1 {
        return 1;
    }

    result[1] = 1;
    result[2] = 2;

    for i in 3..=n {
        result[i as usize] = result[(i-1) as usize] + result[(i-2) as usize];
    }

    result[n as usize]
}

#[test]
fn test_case1 () {
//    assert_eq!(climb_stairs(2), 2);
//    assert_eq!(climb_stairs(3), 3);

    assert_eq!(climb_stairs_dy(2), 2);
    assert_eq!(climb_stairs_dy(3), 3);
}

fn main() {
    println!("Hello, world!");
}
