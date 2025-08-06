// fn greet(){
//     println!("John and Joana are friends")
// }

fn main() {
    let first_name = "John";
    let last_name = "Ogbe";
    let school = "Uniben";
    const matric_no: &str="psc2301010";
    let favourite_emoji = "🤣";
    println!("Hello!, my name is {} {} and I am student of {} with matric no:{} and my favourite emoji is {}",first_name,last_name,school,matric_no,favourite_emoji);
    
    //add
    let add_result = add(10,20);
    println!("{}",add_result);
    
    // subtract
    let subtract_result = subtract(30,20);
    println!("{}",subtract_result);
    
    //multiply
    let multiply_result = multiply(10,20);
    println!("{}",multiply_result);
    
    //divide
    let divide_result = divide(92,3);
    println!("{}",divide_result);
 }
 // Simple maths with rust
    
    // Addition
    fn add(a: i32,b:i32)->i32{
        a + b
    }
    // Subtraction
    fn subtract(a: i32,b:i32)->i32{
         a-b
     
    }
    // divide
    fn divide(a:i32,b:i32)->i32{
        a/b
    }
    // multiply
    fn multiply(a:i32,b:i32)->i32{
        a*b
    }
    
     
