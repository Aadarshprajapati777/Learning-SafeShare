// struct color{
//     red:u32,
//     green:u32,
//     blue:u32
// }


//tuple

struct color(u32, u32, u32);

pub fn run(){
    let mut c= color(255, 0,0);
    println!("{}", c.0);
    
    }