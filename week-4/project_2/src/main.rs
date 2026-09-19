use std::io;

fn main() {
    // 1. Get Experience Input
    println!("Is the employee experienced? (yes/no):");
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input).unwrap();
    let is_experienced = exp_input.trim().to_lowercase() == "yes";

    // 2. Initialize Incentive
    let incentive: u32;

    if is_experienced {
        // Get Age Input only if experienced
        println!("Enter the employee's age:");
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).unwrap();
        let age: u32 = age_input.trim().parse().unwrap_or(0);

        // Determine incentive based on age criteria
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age <= 39 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            // Default fallback for ages 28 and 29
            incentive = 1_300_000; 
        }
    } else {
        // Not experienced
        incentive = 100_000;
    }

    // 3. Print the Result
    println!("The annual incentive is: ₦{}", incentive);
}
