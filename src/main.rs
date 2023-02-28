use std::io;

fn main() {

    // Get user input
    let mut input = String::new();
    println!("Enter how much numbers you want to calculate: ");
    io::stdin().read_line(&mut input).unwrap();

    // Convert to integer
    let max: i32 = input.trim().parse().expect("Not a number");

    // Number crunching time!
    calculate_armstrong(max);
}

fn no_of_digits(mut num: i32) -> i32 {
    let mut i = 0;
    while num > 0 {
        num /= 10;
        i += 1;
    }
    i
}

fn calculate_armstrong(max: i32) {
    let current_time = std::time::Instant::now();
    // Loop through numbers
    for i in 1..=max {
        let mut temp = i; // we need to keep the original number
        let length = no_of_digits(i);
        let mut sum = 0;

        while temp > 0 {
            let digit = temp % 10;
            temp /= 10;
            sum += digit.pow(length as u32);
        }

        if sum == i {
            let elapsed = current_time.elapsed();
            println!("{i} is narcissistic! ({elapsed:?})");
        }
    }
}