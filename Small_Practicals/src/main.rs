/* EXAMPLE - String Slicing
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
        if item == b' '{
            return i;
        }
    }
    s.len()
}
*/
