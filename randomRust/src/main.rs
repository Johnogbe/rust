use std::io;
use rand::seq::SliceRandom;
use rand::Rng;

fn main() {
    // let mut items = vec![1, 2, 3, 4, 5];
    // let mut rng = rand::thread_rng();

    // items.shuffle(&mut rng);
    // println!("Shuffled items: {:?}", items);
    let mut rng = rand::thread_rng();
    let rand_u8: u8 = rng.r#gen();
    let rand_range: u8 = rng.gen_range(1..=10);
    let mut count = 0;
    let mut err_mes : &str= "Guess the correct number from 1 to 10";
    loop{
        println!("{}", err_mes);
        let mut answer = String::new();
        io::stdin().read_line( &mut answer).expect("failed");
        let answer :f32 = match answer.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Enter a valid number");
                continue;
            }
        };

        if answer == rand_range as f32{
            println!("wow you guessed correcrly");
            return;
        }else{
            count +=1;
            err_mes = "Try again";
            if count == 3{
                println!("you failed");
                println!("The answer is:{}",rand_range);

                return;
            }
        }
    }
}
