//use std::thread;
//use std::time::Duration;


// This function is now a variable in generate_workout
// "expensive_closure" 

/*
fn simulated_expensive_calculation(intensity: u32) -> u32{
    println!("calculating slowly... ");
    thread::sleep(Duration::from_secs(2));
    intensity
}
*/

/*
fn main(){
    let intro = String::from("Participant age");
    let print_user_age = |name, age| println!("{}\n\t{}: {}", intro, name, age);

    for(name, age) in [
        (String::from("Alice"), 5),
        (String::from("Bob"), 7),
        (String::from("Mallory"), 20)
    ]
    {
        print_user_age(name, age)
    }
}
*/


/*
fn generate_workout(intensity: u32, random_number: u32){
    //let expensive_result = simulated_expensive_calculation(intensity);
    let expensive_closure = |intensity: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };
    if intensity < 25{
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else{
        if random_number == 3{
            println!("Take a break today! Remember to stay hydrated!");
        } else{
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn main(){
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}
*/


/*

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 20);

for (key, value) in &scores{
  println!("{}: {}", key, value);
}

*/
/*
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct Cacher <'a, T>
where T: Fn(&'a u32) -> &'a u32
{
    calculation: T,
    values: HashMap<&'a u32, &'a u32>
}

impl <'a, T> Cacher <'a, T>
where 
    T: Fn(&'a u32) -> &'a u32 // guess this references the function that is being passed into the Cacher - input should be a reference of intensity var and the output is another u32.
{
    fn new(calculation: T) -> Cacher<'a, T>{ // new instance of Cacher, takes heavy calculation function as argument.
        Cacher{ // instantiate a Cacher struct by passing arguments.
            calculation, // the function passed as T
            values: HashMap::new() // values default to an empty hashmap
        }
    }
    // values function for managing entries in the Cacher struct
    fn values(&mut self, arg: &'a u32) ->&'a u32{
        // match to see if there is an Entry in the Hashmap for value (arg).
        match self.values.entry(arg){
            // there is an entry,
            Entry::Occupied(e) => {
                &*e.into_mut() // returns a mutable reference of the value that occupies the space in the Hashmap ( i guess... )
            }
            // there is no entry.
            Entry::Vacant(e) => {
                // this is pretty confusing
                &*e.insert(&(self.calculation)(&arg)) // this is confusing, I thought it should look more like .insert(key, value)
            }
        }
    }
}

fn generate_workout(intensity:u32, random_number:u32){
    //let expensive_result = simulated_expensive_calculation(intensity);
    // expensive_result as closure
    /*let expensive_result = |intensity:u32| -> u32{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity // return intensity
    };*/

    let mut expensive_result = Cacher::new(|intensity| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });


    if intensity < 25{
        println!(
            "Today, do {} pushups",
            expensive_result.values(&intensity)
        );
        println!(
            "Next, do {} situps",
            expensive_result.values(&intensity)
        );
    } 
    else{
        if random_number == 3{
            // in this case we don't want to run the expensive calculation,
            // which is basically the reason we will implement a closure.
            println!("Take a break today! Remember to stay hydrated!");
        }
        else{
            println!(
                "Today, run for {} minutes",
                expensive_result.values(&intensity)
            );
        }
    }
}

fn main(){
    /*
    let intensity:u32 = 3;
    let random_number = 7;
    //let c = simulated_expensive_calculation(intensity);
    generate_workout(intensity, random_number);
    */
    let mut c = Cacher::new(|intensity| {
        println!("running calculation...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });
    let i:u32 = 3;
    let ii:u32 = 2;
    let ii2:u32 = 2;
    let v1 = c.values(&i);
    let v2 = c.values(&ii);
    let v2_2 = c.values(&ii2);
    println!("V1: {}", v1);
    println!("V2: {}", v2);
    println!("V2.2: {}", v2_2);
}

#[test]
fn call_with_different_values(){
    let mut c = Cacher::new(|a| a);
    let i:u32 = 3;
    let ii:u32 = 2;
    let v1 = c.values(&i);
    let v2 = c.values(&ii);
    println!("V1: {}", v1);
    println!("V2: {}", v2);
}
*/

/*
use std::io;
fn main(){
    let v1:Vec<i32> = vec![1,2,3];
    let v2:Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2,3,4]);

}
#[test]
fn iterator_demonstration(){
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    let g = assert_eq!(v1_iter.next(), None);
    println!("{:?}", g);
}
#[test]
fn iterator_sum(){
    // returns sum of all num elements in a vector
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}
*/

fn main(){
    
}