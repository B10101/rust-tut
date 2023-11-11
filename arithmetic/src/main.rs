use std::io;

fn main() {


    // type casting
    let num = 10 as f32;
    let num2 = 3 as f32;

    let div = (num/num2);
    println!("{}",div);

    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("try again");
    let numbe: f32 = number.trim().parse().unwrap();
    println!("{}", numbe +2.9);


}
