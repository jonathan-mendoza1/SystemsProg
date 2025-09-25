
// Problem 1:
fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
    let mut result = s1.clone();
    result.push_str(s2);
    result
}

// Problem 2:
fn clone_and_modify(s: &String) -> String {
    let mut cloned_word = s.clone();
    cloned_word.push_str("World!");
    cloned_word
}

// Problem 3:
#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    *total = 0;
    for i in low..=high{
        *total += i;
    }
}

fn main() {

    println!("Problem 1 Output:");

    // Problem 1:
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"


    println!();
    println!("Problem 2 Output: ");


    // Problem 2:
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"


    println!();
    println!("Problem 3 Output: ");


    // Problem 3
    // create necessary variables and test your function for low 0 high 100
    let mut total = 0;
    let low = 0;
    let high = 100;
    sum(&mut total, low, high);
    println!("Total: {}", total);
    // total should be 5050
}
