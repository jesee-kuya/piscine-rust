use std::io;

fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess = guess.trim();
        if guess == "The letter e" {
            println!("Number of trials: {count}");
            break;
        }
    }
}
