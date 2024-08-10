// variables are similar to c with the exception of 128 bit signed and unsigned int 
// arch can be used to inherit system instruction size
// fn main(){
//     let name = "Sudhamsh";
//     let age = 24;
//     let height = 5.8;
//     println!("Name :{}\t age:{} \nheight:{}",name,age,height);
// }



//throws error
// fn main() {
//     let interest:f32 = 8;   // integer assigned to float variable
//     println!("interest is {}",interest);
//  }


//separator for easy reading

// fn main(){
//     let a = 123_835.13_001;
//     println!("number :{}",a);
// }

// // char 
// fn main(){
//     let a:char='q';
//     println!("char{}",a);
// }

//mutable
// fn main(){
//     let mut a:i32 = 23_23;
    
//     a+=1;
//     println!("number {}",a);
//     a=32_23;
//     println!("number {}",a);
// }

//const (immutable and type needs to be mentioned )
//shadowing variables
// fn main(){
//     let a:i32 = 34;
//     let a:u32 = 43;
//     const b:i64 = 43;
//     println!("number :{}",a);
//     // const c = 12;//throws error

// }

// &string literal are immutable and static in String objects are stored heap and growable
// fn main(){
//     let s:&str = "Hello";
//     let mut q = String::new();
//     q = "World".to_string();
//     q.push('!');
//     q.push_str("!!");
//     q+="!";
//     println!("{} {} {}",s,q,q.len());
// }
//operators are similar to c

//decision statement
// fn main(){
//     let a:i32;
//     a = 65;
//     if a==100{
//         println!("Number is 100");
//     } 
//     else if a%2==0{
//         println!("even");
//     }
//     else{
//         println!("odd");
//     }
// }

//loop while for
// fn main(){
//     let mut x:i32 = 0;
//     while x<10{
//         if x==0 {
//             x+=1;
//             continue;
//         }
//         println!("x: {}",x);
//         x+=1;
//     }
    
//     for x in 11..31{
//         println!("x: {}",x);
//     }
//     loop{
//         x+=1;
//         if x==50{
//             break;
//         }
//         println!("x: {}",x);
//     }

// }

fn main(){
    let mut v = vec![1,2,3]; 
    // vector v owns the object in heap
 
    //only a single variable owns the heap memory at any given time
    let v2 = v; 
    // here two variables owns heap value,
    //two pointers to the same content is not allowed in rust
 
    //Rust is very smart in terms of memory access ,so it detects a race condition
    //as two variables point to same heap
    v=v2;
    println!("{:?}",v);
}
 
// fn main(){
//     // println!("functions:{}",demo(21));
//     // let mut i:i32 = 48;
//     // println!("value :{}",i);
//     // refer(&mut i);
//     // println!("value :{}",i);
//     // let x:(i32,bool,String) = (21,true,"Sudhamsh".to_string());
//     // tuples_demo(x);
//     // arr_demo();
//     let mut a:[i32;2]=[10,20];
//     println!("{:?}",a);
//     arr_ref(&mut a);
//     println!("{:?}",a);
    
// }
// fn demo(mut x:i32)->f64{
//     println!("int {}",x);
//     x+=1;
//     let mut y:f64 = x as f64;
//     y/3.14

// }
 

// fn refer(point:&mut i32){
//     *point = 46;
// }

//tuples
// fn tuples_demo(x:(i32,bool,String)){
//     println!("{}",x.0);
//     //destructing
//     let (age,gender,name) = x;
//     println!("{} {}, {}",age,gender,name);
// }

// //array demo
// fn arr_demo(){
//     let mut a:[i32;7]=[0;7];

//     for i in 0..a.len(){
//         a[i] = i as i32;
//     }

//     for mut i in 0..a.len(){
//         println!("{}",a[i]);
//         i+=1;
//     }
// }
//array can be passed as reference or value
//size of the array should be known at the compile time
// in other words varaibles can not be used to declare the size of the arr
fn arr_ref(arr:&mut [i32;2]){
    for i in 0..arr.len(){
        arr[i]+=1;
    }
}