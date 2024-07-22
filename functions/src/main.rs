fn main() {
    talk();
    add(2,3.4);
    let result = ret(2,4);
    println!("{}", result)
}

fn talk(){
    println!("Hey you");
}

fn add(x: i32, y: f32){
    let result = (x as f32) + y;
    println!("{}", result);
}

fn ret(x: i32, y: i32) ->i32{
    let result = x + y;
    return result
}