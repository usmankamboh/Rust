fn main(){
    let mut a = 1;
    loop {
        println!("{:?}", a);
        if a == 4 {
            break;
        }
        a = a + 1;
    }

}