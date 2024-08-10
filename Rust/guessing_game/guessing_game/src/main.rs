use std::io;
fn main(){
    let mut num= String::new();
    println!("Enter the number:");
    io::stdin()
    .read_line(&mut num)
    .expect("Failed to read the input");

    println!("You hav entered {:?}",num);
}