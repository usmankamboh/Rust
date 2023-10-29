struct items {
    name : String,
    stock : i32,
    price: f64,
}
fn main() {
    let item1 = items{
        name : "capri".to_string(),
        stock : 10 ,
        price : 123.75,
    };
    println!("name : {:?} ",item1.name);
    println!("stock : {:?} ",item1.stock);
    println!("price : {:?} ",item1.price);
}