
fn assignment1(num: i32) -> i32 {
    // Temp converter
    return (num - 32) * (5/9)
}


fn main() {
    let mut temp = 54;
    temp = assignment1(temp);
    println!("{}", temp);
}
