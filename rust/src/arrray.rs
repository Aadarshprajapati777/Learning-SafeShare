pub fn run(){

    let array:[i32;4]=[1,2,3,4];
    println!("{:?}", array);
    println!("array occupies {} bytes", std::mem::size_of_val(&array));
}