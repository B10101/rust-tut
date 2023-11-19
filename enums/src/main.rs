enum Direction{
        Left,
        Right,
        Up,
        Down,
    }
fn main() {
   let go = Direction::Left;

    match go {
        Direction::Left => println!("Go Left"),
        Direction::Right => println!("Go right"),
        Direction::Up => println!("Go Up"),
        Direction::Down => println!("Go Down"),

    }
}

