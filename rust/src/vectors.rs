use core::num;

pub fn run(){
    let numbers: Vec<i32>= vec![3,2,32,23,2];

    for num in &numbers{
        println!("{}", num);
    }

    println!("number length is {}",numbers.len());
}