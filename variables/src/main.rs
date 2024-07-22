fn main() {

    const PI:f64 = 3.142;

    let mut num:u8 = 2; // 8 bit mutable unsigned integer 
    println!("The number is {}", num);

    num = 4;
    println!("The number is {}", num);

    let imut_num:i32 = -78; // immutale signed integer
    println!("The number is {}", imut_num);

    let is_hot:bool = true; // boolean... either true or false(1 or 0)
    println!("The truth is {}", is_hot);

    let letter:char = 'r'; // character
    println!("The letter is {}", letter);

    let dec:f32 = 4.5678; //32 bit floating point
    println!("The number is {}", dec);
    println!("The number is {}", PI);


    let tup:(u32,f64,bool,char) = (34, PI, false, 'c'); //tuple
    println!("{}",tup.3);


    let arr= [1,2,3,4,5];
    println!("{}", arr[1])


}
