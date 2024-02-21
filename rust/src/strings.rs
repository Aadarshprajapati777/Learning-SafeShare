pub fn run(){
    let mut hello = String::from("hello world");
    let mut s= String::with_capacity(10);
    s.push('a');
    s.push('a');
    println!("{}", s);
    println!("{}",s.capacity());

    assert_eq!(2,s.len());
    assert_eq!(11,s.capacity());

}