//this is about match statements
use std::io;

fn main() {
    let num = 3;
    let mut val = String::new();
    io::stdin().read_line(&mut val).expect("Try again");
    
    match num {
        1 => println!("Its 1"),
        2 => println!("Its 2"),
        3 => println!("Its 3"),
        _ => println!("Its sth else"),

    
    }
   
}
