use std::io;

fn main() {
    println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

    let mut input = String::new();

    let mut trials = 1;

    loop{
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "The letter e" {
            println!("Number of trials: {}", trials);
            break;
        } else {
            trials += 1;
            println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
            input.clear();
        }
    }
}
