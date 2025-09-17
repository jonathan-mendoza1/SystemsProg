
fn assignment1(num: i32) -> i32{
    // Temp converter
    return (num - 32) * (5/9)
}


fn main() {
    let temp = 54;
    let cel = assignment1(temp);
    println!("{}", cel);
}
