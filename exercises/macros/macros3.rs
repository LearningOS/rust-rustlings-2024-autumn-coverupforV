// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


mod macros {
    // 使用 pub 关键字将宏公开
    #[macro_export] // 允许宏在模块外部使用
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    // 使用模块路径调用宏
    my_macro!(); // 直接调用宏，因为 #[macro_export] 将其导出
}
