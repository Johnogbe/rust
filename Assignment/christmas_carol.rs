fn main(){
    let gifts = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a-Laying",
        "Seven Swans a-Swimming",
        "Eight Maids a-Milking",
        "Nine Ladies Dancing",
        "Ten Lords a-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];
    

    for i in 0..gifts.len(){
        println!("On the {} day of Christmas, my true love sent to me:", days[i]);

        for j in (0..=i).rev() {
            if j == 0 && i>0{
            println!("and a {}", gifts[j]);

            }else{
                println!("{}", gifts[j]);

            }
        }
        println!()
    }

}
