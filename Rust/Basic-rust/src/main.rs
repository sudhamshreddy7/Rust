fn datatypes(){
    //below are scaler datatypes
    //int
    //float
    //boolean
    //character

    //compound data types:
    //tuplies
    let t =("Sudhamsh",7);
    //destructuringxs
    let (name,number) =t;
    // println!("{} {}",name,number);
    // println!("{}{}",t.0,t.1);


    //array
    //unlike tuples all elements should be of same type and size cannot be changed
    let mut arr =["Dokuru","AWS","Rust"];
    arr[0]="SUdhamsh";
    // println!("{}{}{}",arr[0],arr[1],arr[2]);
    let arr = ["demo";43];
    for n in 0..43{
        // println!("{}",arr[n]);
    }
}
fn main() {
    // let mut x= 5;
    // println!("value :{}",x);
    // x = 4;
    // println!("value :{}",x);
    // let x = 5.3;
    // println!("value:{}",x);
    // const y : u32= 909_34;
    // println!("value:{}",y);
    // datatypes();
    println!("{}"    ,demo_fun(5,90));
    let exp = true;
    let a=if exp {7} else {8};
    println!("{}",a);
    let mut exp = 0;
    let result = loop{
        exp += 1;
        if(exp==10){
            break exp;
        }
    };
    println!("{}",result);
    let a=[10,20,30,25];
    for element in a.iter(){
        println!("{}",element);
    }
    
}
fn demo_fun(x:i32,y:i32) -> u32{
    println!("{}",x+y);
    (x+y) as u32
}