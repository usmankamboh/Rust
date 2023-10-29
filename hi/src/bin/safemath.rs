fn sum(a:i32,b:i32) -> i32{
    a+b
}
fn sub(a:i32,b:i32) -> i32{
    a-b
}
fn mul(a:i32,b:i32) -> i32{
    a*b
}
fn div(a:i32,b:i32) -> i32{
    a/b
}
fn modu(a:i32,b:i32) -> i32{
    a%b
}
fn display_result(result:i32){
    println!("{:?}",result);
}
fn main(){
    let result =sum(2,2);
    display_result(result);

    let result1 =sub(5,2);
    display_result(result1);

    let result2 =mul(6,2);
    display_result(result2);

    let result3 =div(7,2);
    display_result(result3);

    let result4 =modu(9,2);
    display_result(result4);
}