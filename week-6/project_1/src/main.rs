
use std::io;

fn main() {
    loop {
        println!("\n=== The Shape Calculator ===");
        println!("1. Trapezium (Area)");
        println!("2. Rhombus (Area)");
        println!("3. Parallelogram (Area)");
        println!("4. Cube (Surface Area)");
        println!("5. Cylinder (Volume)");
        println!("6. Exit");
        println!("\nEnter your choice (1-6): ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        if choice == 1 {
            let result = calculate_trapezium_area();
            println!("The area of the trapezium is: {:.2}", result);
        } else if choice == 2 {
            let result = calculate_rhombus_area();
            println!("The area of the rhombus is: {:.2}", result);
        } else if choice == 3 {
            let result = calculate_parallelogram_area();
            println!("The area of the parallelogram is: {:.2}", result);
        } else if choice == 4 {
            let result = calculate_cube_surface_area();
            println!("The surface area of the cube is: {:.2}", result);
        } else if choice == 5 {
            let result = calculate_cylinder_volume();
            println!("The volume of the cylinder is: {:.2}", result);
        } else if choice == 6 {
            println!("Exiting program. Goodbye!");
            break;
        } else {
            println!("Invalid choice! Please select a number between 1 and 6.");
            continue; // Skip the continuation check and show the menu again
        }

        // Ask the user if they want to keep going
        println!("\nDo you want to calculate another shape? (y/n): ");
        let mut keep_going = String::new();
        io::stdin().read_line(&mut keep_going).expect("Failed to read line");
        
        // Clean up the input text to read it accurately
        let keep_going = keep_going.trim().to_lowercase();

        if keep_going == "n" || keep_going == "no" {
            println!("Exiting program. Goodbye!");
            break; // Stop the loop
        }
    }
}

// Helper function to handle individual user inputs safely
fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0.0)
}

// 1. Trapezium Area: height / 2 * (base1 + base2)
fn calculate_trapezium_area() -> f64 {
    println!("\n--- Trapezium Area Calculator ---");
    let height = read_input("Enter height: ");
    let base1 = read_input("Enter base 1: ");
    let base2 = read_input("Enter base 2: ");
    
    (height / 2.0) * (base1 + base2)
}

// 2. Rhombus Area: 1/2 * diagonal1 * diagonal2
fn calculate_rhombus_area() -> f64 {
    println!("\n--- Rhombus Area Calculator ---");
    let d1 = read_input("Enter diagonal 1: ");
    let d2 = read_input("Enter diagonal 2: ");
    
    0.5 * d1 * d2
}

// 3. Parallelogram Area: base * altitude
fn calculate_parallelogram_area() -> f64 {
    println!("\n--- Parallelogram Area Calculator ---");
    let base = read_input("Enter base: ");
    let altitude = read_input("Enter altitude: ");
    
    base * altitude
}

// 4. Cube Surface Area: 6 * side * side
fn calculate_cube_surface_area() -> f64 {
    println!("\n--- Cube Surface Area Calculator ---");
    let side = read_input("Enter side length: ");
    
    6.0 * side * side
}

// 5. Cylinder Volume: pi * radius * radius * height
fn calculate_cylinder_volume() -> f64 {
    println!("\n--- Cylinder Volume Calculator ---");
    let radius = read_input("Enter radius: ");
    let height = read_input("Enter height: ");
    let pi = std::f64::consts::PI;
    
    pi * radius * radius * height
}
