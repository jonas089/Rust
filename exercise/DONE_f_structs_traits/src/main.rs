fn main(){
    /*
    let mut Fluff:Horse = Horse{
        name:String::from("A"),
        position:(0,0,0),
        age:11,
        color:String::from("grey")
    };
    */


    let mut Woopie:Unicorn = Unicorn{
        name:String::from("B"),
        position:(0,0,0),
        age:299,
        color:String::from("white")
    };

    Woopie.fly(0,0,1);
    println!("{} {} {}", Woopie.position.0, Woopie.position.1, Woopie.position.2);

}


struct Horse{
    name:String,
    position: (i32, i32, i32),
    age:u32,
    color:String
}

struct Unicorn{
    name:String,
    position:(i32, i32, i32),
    age:u32,
    color:String
}

trait Movement{
    fn walk(& mut self, x:i32, y:i32);
    fn pos(& mut self) -> (i32, i32, i32);
}

trait Fly{
    fn fly(& mut self, x:i32, y:i32, z:i32); 
}

impl Fly for Unicorn{
    fn fly(& mut self, x:i32, y:i32, z:i32){
        println!("{:?}", self.pos());
    }
}

impl Movement for Unicorn{
    fn walk(& mut self, x:i32, y:i32){
        // apply movement in x and y direction
        self.position = (self.position.0 + x, self.position.1 + y, self.position.2);
    }
    fn pos(& mut self) -> (i32, i32, i32){
        (self.position.0, self.position.1, self.position.2)
    }
}