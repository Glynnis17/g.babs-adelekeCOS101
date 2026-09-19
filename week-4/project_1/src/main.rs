use std::io::{self, Write};

fn main() {
    // 1. Read input for 'a'
    print!("Enter a: ");
    io::stdout().flush().unwrap();
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let a: f64 = input_a.trim().parse().unwrap();

    // 2. Read input for 'b'
    print!("Enter b: ");
    io::stdout().flush().unwrap();
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).unwrap();
    let b: f64 = input_b.trim().parse().unwrap();

    // 3. Read input for 'c'
    print!("Enter c: ");
    io::stdout().flush().unwrap();
    let mut input_c = String::new();
    io::stdin().read_line(&mut input_c).unwrap();
    let c: f64 = input_c.trim().parse().unwrap();

    // 4. Calculate discriminant
    let discriminant = b * b - 4.0 * a * c;

    // 5. Find and print roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two real roots: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One repeated root: {}", root);
    } else {
        println!("The roots are complex/imaginary numbers.");
    }
}
