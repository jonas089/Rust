/* Example 1 - String Slicing
fn main(){
    let mut s = String::from("Hello World");
    let first_word_index = first_word(&s);
    let f_word = &s[0..first_word_index];
    let s_word = &s[(first_word_index + 1)..11];
    println!("{}", f_word);
    println!("{}", s_word)
}
fn first_word(s:&String) -> usize{
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
    // enumerate wraps(wrap,dt. "Kette") the result and iter returns each element as part of a tuple.
        if item == b' '{
            return i;
        }
    }
    s.len()
}
*/
/* Example 2 - Return a Slice
fn main(){
    let mut s = String::from("Hello World");
    let word = first_word(&s);
    println!("{}", word);
}
fn first_word(s:&String) -> &str {
    let bytes = s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
*/

///////////////////////////////////

/* Eample 1 - Function with Struct.
fn main() {
    let width1 = 30;
    let height1 = 50;
    let depth1 = 10;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    println!(
        "The volume of the block is {} m^3 pixels.",
        volume(width1,height1,depth1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn volume(width:u32, height:u32, depth:u32) -> u32{
    width*height*depth
}
*/
/* Example 2 - make use of tuples.
fn main(){
    let rect1 = (30,50);
    println!(
        "The are of the rect is {} square pixels.",
        area(rect1)
    );
}
fn area(dimensions:(u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
*/
/* Example 3 - label data.
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/

///////////////////////////////////
/* Method Syntax - Example 1
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32
}
impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn has_width(&self) -> bool{
        self.width > 0
    }
}
fn main(){
    let rect1 = Rectangle{
        width:30,
        height:50
    };
    println!("The Area of the rectangle is {} square pixels", rect1.area());
    println!("Rectangle has width: {}", rect1.has_width());
}
*/

/* Method Syntax - Example 2*/
struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

fn main(){
    let rect1 = Rectangle{
        width:50,
        height:30
    };
    let rect2 = Rectangle{
        width:20,
        height:10
    };
    let rect3 = Rectangle{
        width:15,
        height:100
    };
    println!("Can rect1 HODL rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 HODL rect3? {}", rect1.can_hold(&rect3));

    println!("{}", Testing_Syntax())
}


fn Testing_Syntax() -> bool{
    let mut is_true:bool = false;
    if(8==16) || (8==7){
        is_true = true;
    }
    is_true
}
