use std::io;

fn main() {
    // and = &&
    //or  = ||
    // not = !
    // order of precedence: !  || &&

    let cond = (2 as f32) > 3.4;
    println!("{}", cond);

    let food = "fruit";
    if food == "bread"{
        println!("Pretty nice");

    }
    else if food == "fruit" {
        println!("Pretty healthy");
    }
    else{
        println!("JUNK");
    }
}
