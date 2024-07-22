use std::io;

fn main() {


    // type casting
    let num = 10;
    let num2 = 3;
    let div = num as f32 / num2 as f32;
    println!("{}",div);

    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("try again");
    let numbe: f32 = number.trim().parse().unwrap();
    println!("{}", numbe +2.9);


}
