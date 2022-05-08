# 4.3 - Slices [UNSOLVED]

How to efficiently slice/return the second_word ?

# 6.1 - How to access the string value of the Write function? [SOLVED]
Question: How would one pass the value that is to be written
```python
#[derive(Debug)]
enum Message{
    //Quit,
    Move{x:i32, y:i32},
    Write(String),
    //ChangeColor(i32, i32, i32)
}

fn Write(msg:String){
    println!("FUNCTION CALLED.");
}

impl Message{
    fn call(&self){
        println!("{:?}", self);
    }
}
```
## ANSWER

user structs together with enums:

```python
#[derive(Debug)]
enum Gender{
    Male,
    Female
}
#[derive(Debug)]
struct Player{
    name: String,
    gender: Gender
}
fn main(){
    let player_1 = Player{
        name:String::from("Jonas"),
        gender: Gender::Male
    };
    let player_2 = Player{
        name:String::from("Kristiana"),
        gender: Gender::Female
    };

    println!("Player 1: {:#?}", player_1);
    println!("Player2: {:#?}", player_2);

    println!("{}", player_1.name)
}
```
