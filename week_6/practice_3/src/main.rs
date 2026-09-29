fn main() {
    let name1 = String::from("Ayomide Lawal");
    let new_name = name1.replace("Ayomide", "Adebare");

    let faculty = String::from("Faculty of Science");
    let new_faculty = faculty.replace("Faculty", "School");

    println!("Original: {}", name1);
    println!("Changed: {}", new_name);
    println!("Changed Faculty: {}", new_faculty);
}
