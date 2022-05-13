use LargerProject::front_of_house::hosting;
mod import;
fn main() {
    println!("Hello, world!");
    import::hello_external();
    hosting::add_to_waitlist();
}
