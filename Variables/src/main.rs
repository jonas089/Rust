fn main() {
    //let x = plus_one(5);
    //println!("The value of x is: {}", x);
    //reverser(5);
    Ownership();
}
fn plus_one(x:i32) -> i32{
    x + 1
}
fn reverser(y:i32){
    for number in (1..y).rev(){
        println!("{}!", number);
    }
    println!("Done!");
}
fn Ownership(){
    let mut r = "hello";
    let mut s = String::from(r);
    s.push_str(", world!");
    println!("{}",s);

}
