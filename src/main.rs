use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello!");
    println!("Please guess a number");
    let secret = rand::thread_rng().gen_range(1..=10);
    
    loop{
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    match guess.cmp(&secret){
        Ordering::Less=>println!("Too Less"),
        Ordering::Greater=>println!("Too large"),
        Ordering::Equal=>{
            println!("Correct");
            break;}
    }
}

}
