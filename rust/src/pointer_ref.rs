pub fn run(){
    // let arr1=[1,2,3,4,5,5];
    // let arr2=[0,00,0,0,0,0];
    // let arr1=arr2;

    // println!("{:?}, {:?}", arr1,arr2);

    let arr1:Vec<i32>=vec![1,2,3,4,5,5];
    let arr2=vec![0,00,0,0,0,0];
    let arr1=&arr2;

    println!("{:?}, {:?}", &arr1,&arr2);

}