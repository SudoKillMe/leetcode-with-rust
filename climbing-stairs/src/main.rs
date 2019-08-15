
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

#[test]
fn test_case1 () {
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
}

fn main() {
    println!("Hello, world!");
}
