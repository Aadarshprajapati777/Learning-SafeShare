enum Movement{
    Up,
    Left,
    Right,
    Down
}


fn avtar_movement(m:Movement){
    match m {
        Movement::Up=> println!("avtar is moving up"),
        Movement::Down=> println!("Avatar is moving down"),
        Movement::Left=> println!("Avatar is moving left"),
        Movement::Right=> println!("Avatar is moving right")
    }
}





pub fn run(){
    let avatar1= Movement::Left;
    let avatar2= Movement::Up;
    let avatar3=Movement::Down;
    let avatar4=Movement::Right;

    avtar_movement(avatar1);   
    avtar_movement(avatar2);   
    avtar_movement(avatar3);   
    avtar_movement(avatar4);   

}