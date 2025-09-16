use std::io;
use std::fs;
use std::collections::HashMap;
use std::process;
use serde_json;
use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct Definitions{
    #[serde(flatten)]
    definitions:HashMap<String,Vec<String>>
}
fn main() -> Result<(),Box<dyn std::error::Error>>{
    println!("Welcome. type in word press EXIT to cancel");
     
     let word = fs::read_to_string("data.json")?;
    loop {
        println!("Search another word. Exit to cancel");
        let mut search = String::new();
       
        io::stdin().read_line(&mut search).expect("Could not read input");
        search = search.trim().to_string();
        if search == "EXIT".to_string() {
           //    return Err("exit".into());
           //break;
           process::exit(0);
           
          };
          
       search = search.trim().to_lowercase().to_string();
   
        let definitions:Definitions = serde_json::from_str(&word)?;
        for (word,meaning) in definitions.definitions{
            if word.to_lowercase().to_string() == search{
                for meaning in meaning {
                    println!("Meaning: {}", meaning);
    
                };
                
            }
        };
        //  if let Some(meanings) = definitions.definitions.get(&search){
        //     for meaning in meanings{
        //         println!("{}",meaning)
        //     }
        //  }else {
        //      print!("Not found")
        //  }
    };

 
    //Ok(())

}

// fn input()->String{
//      let mut search = String::new();
       
//      io::stdin().read_line(&mut search).expect("Could not read input.");
//      let search = search.trim();
//      if search == "EXIT" {
//         //    return Err("exit".into());
//         process::exit(0)
//        }
       
//     return search.trim().to_lowercase()
// }
