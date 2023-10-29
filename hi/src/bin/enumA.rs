enum colors{
    black,
    blue,
    red,
    pink,
    purple,
}
fn print_color(my_color:colors){
    match my_color {
        colors::black => print!("black"),
        colors::blue => print!("blue"),
        colors::pink => print!("pink"),
        colors::purple => print!("purple"),
        colors::red => print!("purple"),
    }
}
fn main() {
    print_color(colors:: pink);
}