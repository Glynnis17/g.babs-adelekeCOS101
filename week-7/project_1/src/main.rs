use std::io::{self, Write};

fn main() {
    let aps_matrix = vec![
        ["APS 1-2", "Intern", "-", "Paralegal", "Placement"],
        ["APS 3-5", "Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"],
        ["APS 5-8", "Senior Administrator", "PhD Candidate", "Associate", "Senior Teacher"],
        ["EL1 8-10", "Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"],
        ["EL2 10-13", "Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"],
        ["SES", "CEO", "Dean", "Partner", "Principal"],
    ];

    println!("--- APS Level Checker Tool ---");
    println!("Type 'exit' at any time to close the program.\n");

    // 1. The main program loop starts here
    loop {
        print!("Enter a job title: ");
        io::stdout().flush().unwrap(); 

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let search_role = user_input.trim().to_lowercase();

        // 2. Check if the user wants to leave the loop
        if search_role == "exit" {
            println!("Goodbye!");
            break; // Exits the loop block entirely
        }

        // 3. Perform the search
        let mut found_level = None;
        for row in &aps_matrix {
            for item in row {
                if item.to_lowercase() == search_role {
                    found_level = Some(row[0]); // Grab the Level string at index 0
                }
            }
        }

        // 4. Output results using unwrap_or
        let final_level = found_level.unwrap_or("Role not found in the APS system");
        println!("APS Position Level: {}\n", final_level);
        
        // Loop automatically cycles back up to the top here!
    }
}
