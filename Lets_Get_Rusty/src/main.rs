/* Without a lifetime specified, the variable is wiped when it's scope is left.
fn main(){
    let reference_to_nothing: &String = dangle();
}
fn dangle() -> &String{
    let s:String = String::from("hello");
    &s
}
*/

fn main(){
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );
}
fn area(width:u32, height:u32) -> u32{
    width * height
}






/*
struct User{
    username:String,
    email: String,
    sign_in_count: u64,
    is_active: bool
}

#[test]
fn _structs(){
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let mut User1 = User{
        email: String::from("jonas@mail.com"),
        username: String::from("jonas"),
        is_active: true,
        sign_in_count: 1
    };

    let name: &String = &User1.username;
    User1.username = String::from("new username");

    let User2: User = build_user(
        String::from("Kristiana@email.en"), 
        String::from("Kristiana"));
}

fn build_user(email: String, username: String) -> User {
    User{
        email: email,
        username: username,
        is_active: true,
        sign_in_count: 1
    }
}
*/

/*
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
*/
/*
#[test]
fn example_01(){
    let s1: String = String::from("hello");
    let len:usize = calculate_length(&s1);
    println!("The length of {} {}", s1, len);
}

fn calculate_length(s: &String) -> usize{
    let length:usize = s.len();
    length
}

*/

