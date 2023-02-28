use std::io;
use std::env;

fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();

    // Check if there is a --fast argument
    let fast = args.contains(&"--fast".to_string());
    if fast {
        println!("Fast mode enabled");
    } else {
        println!("Fast mode not enabled. To enable it use --fast (cargo run -- --fast)");
    }

    // Get user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Convert to integer
    let max: i32 = input.trim().parse().expect("Not a number");

    match fast {
        true => fast_narcissistic(max),
        false => narcissistic(max),
    }


}

fn fast_narcissistic(max: i32) {
    // Loop through numbers
    for i in 1..max {
        // Calculate if it is a narcissistic number
        let mut sum = 0;
        let mut num = i;
        while num > 0 {
            let digit = num % 10;
            // sum += digit.pow(3);
            // we need to use the power of the number of digits
            // so we need to calculate the number of digits
            let mut digits = 0;
            let mut temp = i;
            while temp > 0 {
                temp /= 10;
                // digits += 1;
                // to fix one digit numbers
                if i > 9 {
                    digits += 1;
                }
            }
            sum += digit.pow(digits as u32);
            num /= 10;
        }
        // Print if it is
        if sum == i {
            println!("{}", i);
        }
    }
}

// The same version but with logging variables (total calculations)
fn narcissistic(max: i32) {
    // Loop through numbers
    let mut calculations: u64 = 0;
    for i in 1..max {
        // Calculate if it is a narcissistic number
        let mut sum = 0;
        let mut num = i;
        while num > 0 {
            let digit = num % 10;
            calculations += 1;
            // we need to use the power of the number of digits
            // so we need to calculate the number of digits
            let mut digits = 0;
            let mut temp = i;
            while temp > 0 {
                temp /= 10;
                calculations += 1;
                // digits += 1;
                // to fix one digit numbers
                if i > 9 {
                    calculations += 1;
                    digits += 1;
                    calculations += 1;
                }
            }

            sum += digit.pow(digits as u32);
            calculations += 1;
            num /= 10;
            calculations += 1;
        }
        // Print if it is
        if sum == i {
            println!("{}", i);
        }
    }
    println!("Calculations: {}", calculations);
}