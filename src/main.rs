use std::io;
fn main() {
    let mut input = String::new();
    let mut a =0;
    let mut b =1;
    let mut c =0;
    let mut temp =0;
    println!("enter length of fibonacci!");
    io::stdin().read_line(&mut input).expect("failed to read");
    a=input.trim().parse::<i32>().unwrap();
    for i in 1..a{
        println!("{} comes {}",i,temp );
        temp=b+c;
        b=c;
        c=temp;
    }
}
