enum Direction {
    LEFT,
    RIGHT,
    FORWARD
}
fn main() {
    let go = Direction::LEFT;
    match go {
        Direction::LEFT => println!("go left"),
        Direction::RIGHT => println!("go right"),
        Direction::FORWARD => println!("go forward"),
    }
}