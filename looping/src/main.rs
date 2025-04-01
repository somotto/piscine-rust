use std::io::{self, Write};

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "the letter e";
    let mut attempts = 0;
    
    loop {
        println!("{}", riddle);
        
        io::stdout().flush().unwrap();
        
        let mut user_answer = String::new();
        io::stdin()
            .read_line(&mut user_answer)
            .expect("Failed to read line");
        
        attempts += 1;
        
        let user_answer = user_answer.trim().to_lowercase();
        
        if user_answer == answer || user_answer == "e" {
            println!("Number of trials: {}", attempts);
            break;
        }
    }
}