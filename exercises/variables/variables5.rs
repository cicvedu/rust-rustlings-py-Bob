// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut number = "T-H-R-E-E";
    println!("Spell a Number : {}", number);
    number = "3"; // 将数字3改为字符串"3"
    let parsed_number:i32= number.parse().unwrap(); // 字符串转换为数字
    println!("Number plus two is : {}", parsed_number + 2);
}