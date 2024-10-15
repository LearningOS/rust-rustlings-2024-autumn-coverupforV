//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // 在 tests7 中，我们应该设置一个名为 `TEST_FOO` 的环境变量。
    // 让 Cargo 根据输出设置它。
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 设置 TEST_FOO 为当前时间戳
    println!("cargo:TEST_FOO={}", timestamp);

    // 在 tests8 中，我们应该启用 "pass" 特性以使测试用例提前返回。
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

