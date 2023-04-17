// 1. 每个值都有一个所有者（owner）。
// 2. 同一时间只能有一个所有者。
// 3. 当所有者离开作用域时，它所拥有的资源将被自动释放。

pub fn use_ownership() {
    // String s1 = new String("hello");
    let s1 = String::from("hello");
    // 变量s1的算有权以及被转移至s2，因此s1能再被访问
    let s2 = s1;

    // ERROR : println!("s1: {}", s1);
    println!("s2: {}", s2);
}

pub fn use_fn_ownership() {
    // 定义s，这里s不是基本类型
    let s = String::from("hello");
    // 定义函数takes_ownership，这里s的所有权被转移给了takes_ownership
    takes_ownership(s);
    // ERROR : println!("s: {}", s);
    // 定义x为基本类型
    let x = 5;
    // 定义函数makes_copy，这里x的所有权被复制给了makes_copy
    makes_copy(x);
    // x可以继续使用
    println!("x: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("some_string: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer: {}", some_integer);
}