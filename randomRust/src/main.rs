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
    let mut count = 0;
    loop{
        println!("guess");
        let mut answer = String::new();
        io::stdin().read_line( &mut answer).expect("failed");
        let answer :f32 = match answer.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Enter a valid number");
                continue;
            }
        };

        if answer == rand_u8 as f32{
            println!("wow you guessed correcrly");
            return;
        }else{
            count +=1;
            if count == 3{
                println!("you failed");
                println!("The answer is:{}",rand_u8);

                return;
            }
        }
    }
}
