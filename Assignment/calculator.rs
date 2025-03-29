// Calculator works well but the logic can be greatly improved i will work on it again
use std::io;
fn choice()->String{
    
println!("Normal calculations(press A) or cgpa calculations?(press B)");
let mut option = String::new();

    io::stdin().read_line(&mut option).expect("failed to read line");
    option.trim().to_string()
    }
fn main(){
    println!("Welcome to Calculator.com");

   



    let mut option = choice();
    while option != "A" && option != "B"{
       option= choice()
    }
    if option == "A" {
        println!("You chose Normal Calculations!");
        calc()
    } else if option == "B" {
        println!("You chose CGPA Calculations!");
        gpa()
    }

}
fn calc(){
    let mut in_1 = String::new();
    let mut in_2 = String::new();
    let mut sign = String::new();
    println!("Input First no");
    io::stdin().read_line(&mut in_1).expect("failed to read line");
    println!("Input second no");
    io::stdin().read_line(&mut in_2).expect("failed to read line");
    println!("Input operator.+,_,*");
    io::stdin().read_line(&mut sign).expect("failed to read line");

    let no_1:f32 = in_1.trim().parse().expect("invalid no");
    let no_2:f32 = in_2.trim().parse().expect("invalid no");
    let sign = sign.trim();
    let result = match sign {
        "+"=>no_1+ no_2,
        "-"=>no_1- no_2,
        "*"=>no_1* no_2,
        "/"=>{
                if no_2 == 0.0{
                    println!("cannot divide by 0");
                    return;
                }else{
                    no_1/no_2
                }
        },
        _=> 0.0
    };
    println!("Result of {} {} {} is {}",no_1,sign,no_2,result);
}
fn gpa(){
    let mut courses:Vec<String> = vec![];
    let mut scores:Vec<i32> = vec![];
    let mut loads:Vec<i32> = vec![];

    let mut c_u:f32=0.0;
    let mut q_p:f32=0.0;
    println!("Welcome> how many courses do you offer?");
    let mut no_courses = String::new();
    io::stdin().read_line(&mut no_courses).expect("error couldnt read input");
    let nu_courses:usize = no_courses.trim().parse().expect("did not convert");
    for i in 0..nu_courses{
        
            println!("Enter course code for number {}",i+1);
        

    let mut course_name = String::new();
    io::stdin().read_line(&mut course_name).expect("error couldnt read input");
    courses.push(course_name.trim().to_string())
    }
    for i in 0..courses.len(){
        println!("Enter score for {} course",courses[i]);
    let mut str_score= String::new();
    io::stdin().read_line(&mut str_score).expect("error couldnt read input");
    let mut score:i32 = str_score.trim().parse().expect("couldnot conver");
    scores.push(score);
    
    }
    for j in 0..courses.len(){
        println!("Enter course load/unit {}",courses[j]);
    let mut load= String::new();
    io::stdin().read_line(&mut load).expect("error couldnt read input");
    let mut nu_load:i32 = load.trim().parse().expect("couldnot conver");
    loads.push(nu_load);
    };
    for p in 0..courses.len(){
        let grade:f32 = match scores[p]{
            0..=39=>0.0,
            40..=44=>1.0,
            45..=49=>2.0,
            50..=59=>3.0,
            60..=69=>4.0,
            70..=100=>5.0,
            _=>{
                println!("Invalid score {}",scores[p]);
                continue;
            }
        };
    q_p += grade * loads[p] as f32;
    c_u +=  loads[p] as f32;
    
    };
    let gpa:f32 = q_p/c_u;
    println!("your GPA ia {}",gpa)
}