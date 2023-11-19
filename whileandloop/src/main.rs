fn main() {
    loopy();
    whily();
}

fn loopy(){
    let mut i = 1;
    loop {
        println!("{:?}", i);
        if i == 10 {
            break;
        }
        i = i + 1;
    }  
}

fn whily(){
    let mut i = 10;
    while i >= 0 {

        println!("{:?}", i);
        i = i - 1;

    }
    println!("DONE");
}
