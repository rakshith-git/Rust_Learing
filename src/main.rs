// Ive created a user input based fibonacci series program in rust!!!

use std::io;
fn main() {
    let mut input = String::new();
    let mut a : i128 =0;
    let mut b : i128 =1;
    let mut c : i128 =0;
    let mut temp : i128 =0;
    println!("enter length of fibonacci!");
    io::stdin().read_line(&mut input).expect("failed to read");
    a=input.trim().parse::<i128>().unwrap();
    for i in 1..a{
        println!("{} comes {}",i,temp );
        temp=b+c;
        b=c;
        c=temp;
    }
}
