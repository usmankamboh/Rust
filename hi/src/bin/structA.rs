enum flavor {
    sparkling,
    sweet,
    fruity,
}
struct drink {
    flavord : flavor,
    fluid_oz : f64,
}
fn print_drink (drinked : drink){
    match drinked.flavord{
        flavor::fruity => println!("flaovr : fruity"),
        flavor::sparkling => println!("flavour :sparklimg"),
        flavor::sweet => println!("flavour : sweet"),
    }
    println!("oz : {:?}", drinked.fluid_oz);
}
fn main() {
    let taste_sweet = drink{
        flavord:flavor::sweet,
        fluid_oz : 6.0
    };
    print_drink(taste_sweet);
    let taste_fruity = drink{
        flavord:flavor::fruity,
        fluid_oz : 10.0
    };
    print_drink(taste_fruity);
}