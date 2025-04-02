use std::io;
fn main() {
    /*
     * A rust program that:
     * - Ask a user how many students are in the class (min 10)
     * - For each user, prompt them to give their name and score
     * - For loop to process each students score
     * - Use control flow to assign grade based on the score
     * - - 70-100 - A
     * - - 60-69 - B
     * - - 55-59 - C
     * - - 45-54 - D
     * - - 40-45 - E
     * - - 0-39 - F
     */
println!("Welcome. How many students are in class?");
let mut number = call();
    // Ensure the number of students is at least 10
    while number <= 9 {
        println!("Number of students must be atleast 10");
       number =  call();
    }

    struct Students {
        name:String,
        score:f32
    }
    let mut store_records: Vec<Students>= Vec::new();
    for i in 1..=number{
        println!("Name of student {}",i);
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("error");
        let name = name.trim().to_string();
        println!("score for {}",name);
        let mut score = String::new();
        
        let mut score_convert:f32;
        loop{
        io::stdin().read_line(&mut score).expect("error");
        match score.trim().parse::<f32>(){
            Ok(s)=>{
                score_convert = s;
                break
            }
            Err(_)=>{
                println!("Invalid input. Please enter a valid score for {}", name);
                score.clear();
            }}
        
        
        }
        store_records.push(Students{
            name:name,
            score:score_convert
           })
    }
    let mut i = 0;
println!("\nStudents Record ------------------------------------");
    for student in store_records.iter(){
        
        let grade = if student.score > 0.00 && student.score <=39.00{
             "F"
        }else if student.score >= 40.00 && student.score <= 44.00{
             "E"
        }else if student.score >= 45.00 && student.score <=  49.00{
             "D"
        }else if student.score >= 50.00 && student.score <=  59.00{
             "C"
        }else if student.score >= 60.00 && student.score <=  69.00{
             "B"
        }else{
             "A"
        };
        i = i + 1;
        println!("{}. name: {}      score: {}      grade: {}\n",i, student.name, student.score,grade);
    }
}
fn call() -> u32{
    let mut no_of_students = String::new();
    io::stdin().read_line(&mut no_of_students).expect("Error reading input");
    
    // Parse the number of students into u32
    let  no_of_students: u32 = no_of_students.trim().parse().expect("Error converting to number");
    no_of_students
}