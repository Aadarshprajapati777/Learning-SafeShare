// // struct color{
// //     red:u32,
// //     green:u32,
// //     blue:u32
// // }


// //tuple

// struct color(u32, u32, u32);

// pub fn run(){
//     let mut c= color(255, 0,0);
//     println!("{}", c.0);

//     }



struct Person{
    first_name: String,
    last_name: String,
    age: u32,
    id: i32
}

impl Person {
    fn new(first_name: &str, last_name:&str, age:u32, id:i32)->Person{
        Person{
            first_name: first_name.to_string(),
            last_name:last_name.to_string(),
            age:age,
            id: id
        }
    }


fn get_fullname(&self) -> String{
    format!("{} {}", self.first_name, self.last_name)
}

fn set_lastname(&mut self, last:&str){
    self.last_name=last.to_string();
}
}




pub fn run(){

    let mut new_person= Person::new("Aadarsh", "Prajapati", 21, 01);
    println!("{} ", new_person.get_fullname());

    new_person.set_lastname("Kohar");

    println!("{}", new_person.get_fullname());

}