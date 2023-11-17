fn main() {
    let user1 = User{
        name: String::from("bett"),
        age:23,
        gender: String::from("M"),
        active: true, 
    };

    println!("{}, {}",user1.name,user1.age)
}
struct User{
    name: String,
    age: u32,
    gender:String,
    active:bool,
    
}
