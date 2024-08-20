use std::io;
use rand::{random, Rng};
use std::cmp::Ordering;
fn main() {
    println!("Enter the number:");
    let mut num: String=String::new();
    io::stdin()
    .read_line(&mut num)
    .expect("Failed to read line");
    let a: i64 = num
    .trim()
    .parse()
    .expect("ENter valid number");
    let x:i64 = rand::thread_rng().gen_range(1,101);
    println!("the secretn number:{}",x);
    println!("The guessed number is :{}",a);
    match a.cmp(&x){
        Ordering::Less => println!("small"),
        Ordering::Equal => println!("correct"),
        Ordering::Greater => println!("greater"),
    }
}
