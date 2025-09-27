// Problem 1:
const FREEZING_POINT :f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    let conv = (f - FREEZING_POINT) * 5.0/9.0;
    conv
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    let conv = (c * 9.0/5.0) + FREEZING_POINT;
    conv
}


// Problem 2:
fn is_even(n: i32) -> bool {
    if n % 2 == 0{
        return true;
    }
    return false;
}


// Problem 3:
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret{
        return 0;
    }
    else if guess > secret{
        return 1;
    }
    else{
        return -1;
    }
}

fn main() {

    println!("Problem 1:");
    println!();
    
    
    // Problem 1:
    let mut temp_fah: f64 = 54.0;
    let temp_cel = fahrenheit_to_celsius(temp_fah);
    println!("Fahrenheit to Celsius: ");
    println!("{}°F --> {:.2}°C", temp_fah, temp_cel);

    for _ in 0..5 {
        temp_fah += 1.0;
        let temp_cel = fahrenheit_to_celsius(temp_fah);
        println!("{}°F --> {:.2}°C", temp_fah, temp_cel);
    }
    
    let mut temp_c: f64 = 15.0;
    let temp_f = celsius_to_fahrenheit(temp_c);
    println!("Celsius to Fahrenheit: ");
    println!("{}°C --> {:.2}°F", temp_c, temp_f);

    for _ in 0..5 {
        temp_c += 1.0;
        let temp_f = celsius_to_fahrenheit(temp_c);
        println!("{}°C --> {:.2}°F", temp_c, temp_f);
    }


    println!();
    println!("Problem 2: ");
    println!();


    // Problem 2:
    let array: [i32; 10] = [34, 64, 12, 75, 35, 75, 23, 54, 16, 42];

    for i in array{
        if is_even(i){
            print!("{} is even", i);
        }
        else{
            print!("{} is odd", i)
        }

        if i % 3 == 0 && i % 5 == 0{
            println!(", Fizzbuzz")
        }
        else if i % 3 == 0{
            println!(", Fizz")
        }
        else if i % 5 == 0{
            println!(", Buzz")
        }
        else{
            println!()
        }
    }

    let mut total = 0;
    let mut i = 0;
    while i < array.len() {
        total += array[i];
        i += 1;
    }
    println!("Sum of elements in array: {}", total);

    let mut max = array[0];
    let mut j = 0;
    loop{
        if max < array[j]{
            max = array[j];
        }
        j += 1;
        if j == array.len(){
            break;
        }
    }
    println!("Max element in array: {}", max);


    println!();
    println!("Problem 3: ");
    println!();


    // Problem 3:
    let secret = 7;
    let mut guess = 4;
    let mut guess_count = 0;

    loop{
        guess_count += 1;
        let attempt = check_guess(guess, secret);

        if attempt == 0{
            println!("You guessed correct! The secret number is {}!", secret);
            break;
        }
        else if attempt == 1{
            println!("{} is too high, try again!", guess);
            guess -= 1;
        }
        else{
            println!("{} is too low, try again!", guess);
            guess += 1;
        }
    }

    println!("Amount of guesses: {}", guess_count);
    println!()
}
