// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 42;
    let option = Some(12);

    // 使用 if let 处理 Option
    if let Some(x) = option {
        res += x;
    }

    // 或者，使用 map 方法
    // res += option.map_or(0, |x| x); // 如果 option 为 None，则使用 0

    println!("{}", res);
}
