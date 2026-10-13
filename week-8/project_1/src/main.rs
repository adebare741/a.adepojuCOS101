use std::io;

fn main() {
    // APS table: (role, min_years, max_years, level)
    let aps_table = vec![
        ("Intern", 0, 2, "APS 1-2"),
        ("Paralegal", 0, 2, "APS 1-2"),
        ("Placement", 0, 2, "APS 1-2"),

        ("Administrator", 3, 5, "APS 3-5"),
        ("Research Assistant", 3, 5, "APS 3-5"),
        ("Junior Associate", 3, 5, "APS 3-5"),
        ("Classroom Teacher", 3, 5, "APS 3-5"),

        ("Senior Administrator", 5, 8, "APS 5-8"),
        ("PhD Candidate", 5, 8, "APS 5-8"),
        ("Associate", 5, 8, "APS 5-8"),
        ("Senior Teacher", 5, 8, "APS 5-8"),

        ("Office Manager", 8, 10, "EL1 8-10"),
        ("Post-Doc Researcher", 8, 10, "EL1 8-10"),
        ("Senior Associate 1-2", 8, 10, "EL1 8-10"),
        ("Leading Teacher", 8, 10, "EL1 8-10"),

        ("Director", 10, 13, "EL2 10-13"),
        ("Senior Lecturer", 10, 13, "EL2 10-13"),
        ("Senior Associate 3-4", 10, 13, "EL2 10-13"),
        ("Deputy Principal", 10, 13, "EL2 10-13"),

        ("CEO", 13, 40, "SES"),
        ("Dean", 13, 40, "SES"),
        ("Partner", 13, 40, "SES"),
        ("Principal", 13, 40, "SES"),
    ];

    // Ask user for role
    println!("Enter your role:");
    let mut role = String::new();
    io::stdin().read_line(&mut role).expect("Failed to read input");
    let role = role.trim(); // remove newline

    // Ask user for years of experience
    println!("Enter your years of experience:");
    let mut years_str = String::new();
    io::stdin().read_line(&mut years_str).expect("Failed to read input");
    let years: i32 = years_str.trim().parse().unwrap_or(0);

    // Check role and years against APS table
    let mut found = false;
    for (r, min, max, level) in &aps_table {
        if r == &role && years >= *min && years <= *max {
            println!("{} with {} years of experience is {}", role, years, level);
            found = true;
            break;
        }
    }

    if !found {
        println!("{} with {} years not found in APS table", role, years);
    }
}
