use std::env;

fn main() {
    // 获取环境变量的值并解析为 u64 类型
    let env_var = env::var("TEST_FOO").unwrap();
    let value: u64 = env_var.parse().unwrap();

    // 验证环境变量的值是否在指定的范围内
    assert!(value >= 100 && value <= 200);
}