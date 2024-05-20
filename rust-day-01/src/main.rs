use std::ffi::CString;

fn main(){
    //  let  a :u32 = 16;
    //  let b = -16;
    //  let c = 1.6;
    //  let chars = 'c';
    //
    //  let true_value = true;
    //
    // let tuple = ( 1,3,-3, 'c',"String");
    //
    // println!("the valus are {} {} {} {} {} {:?}",a,b,c,chars,true_value,tuple);
    //
    //  let array = [2,4,5,6,7,8,0];
    //  println!("{:?}",array);
    //
    //  let mut  string_name = "mishal@gmail.com";
    //
    // println!("name is:{}",string_name);


    let  mut string = String::from("hello");
    let mut  x = &mut string;
    x.push_str(" world");
    println!("x is :{}",x);
    println!("string is :{}",string);



}

fn modify_string(s:&mut String){
    s.push_str(" world");
}
fn print_string(s: &String){
    println!("s {}",s);
}