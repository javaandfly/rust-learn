

pub fn debug_ownership<>() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效                       
                                    
    // println!("在move进函数后继续使用s: {}",s); //编译器会报错 因为s已经转移了所有权

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_print_vec() {
        // let value = vec![1, 2, 3, 4];
        debug_ownership();
        // 添加断言来验证输出内容
        // assert_eq!(value, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_debug_print_str() {
        // let message = "Hello, Rust!";
        // debug_ownership(&message);
        // 添加断言来验证输出内容
        // assert_eq!(message, "Hello, Rust!");
    }

    // 你可以在这里添加更多的测试函数
}