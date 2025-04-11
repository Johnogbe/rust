use std::io;
fn main(){
    println!("Enter expression");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed");
    let input = input.trim();

    let mut numbers:Vec<i32> = Vec::new();
    let mut operations:Vec<char> = Vec::new();
    let mut build_digit_temp = String::new();

    for c in input.chars(){
        if c.is_digit(10){
            build_digit_temp.push(c)
        }else if !build_digit_temp.is_empty(){
            numbers.push(build_digit_temp.parse().unwrap());
            build_digit_temp.clear();
            if "+-/*".contains(c){
            operations.push(c)
            }
        }else if "+-/*".contains(c){
            operations.push(c)
        }
        
    }
    if !build_digit_temp.is_empty() {
        numbers.push(build_digit_temp.parse().unwrap());
    }
    if !numbers.is_empty(){
        let mut result = numbers[0];
        for(i, &sign) in  operations.iter().enumerate(){
           if let Some(&num) = numbers.get(i +1) {
                 match sign {
                '+'=>result +=num,
                '-'=>result -=num,
                '*'=>result *=num,
                '/'=>if num == 0 {println!("cant divide by 0");return;}else{result /=num}
                 _ =>(),
                 }
            }
        }
        println!("Final result is {}",result);
    }  else{
    println!("No Number inputed")

    }  
}
