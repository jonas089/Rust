
fn main(){

}

#[test]
fn _ownership_example() {
    let x: i32 = 5;
    let y: i32 = x;
    // using multiple variables without changing ownership.
    let s1: String = String::from("hello");
    let mut s2: &String = &s1; // creates a mutable reference of s1
    let mut s3: String = s1.clone() + "hello"; // create an actual copy of variable s1
    println!("{} {} {}", s1, s2, s3);
    assert_eq!(String::from("hellohello"), s3);
    assert_eq!(String::from("hello"), s1);

    let mut s4 = s1; // takes ownership of s1 and s1 is not valid from here.
    assert_eq!(String::from("hello"), s4);

    // passed.
}
