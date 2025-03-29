use std::io;
fn main(){
    
    let mut state = false;
    while !state {
        let mut input = String::new();
    println!("Enter value. must be 100F");
    io::stdin().read_line(&mut input).expect("failed to read line");
    let number: f32 = input.trim().parse().expect("invalid no");
    if number != 100.0 {
        println!("number must be 100👌")
        
    }else{let answer = (number - 32.0) * 5.0 / 9.0;
        println!("{:?}", answer);
        state = true
    }
}
}
    
