fn main() {

    //varibales data types -> Rust is a type language
    
    let x= 16;
    println!("{}",x);

    let z: String = String::from("Hello, Soroban!");
    let y :&str ="Hello Soroban!";

    println!("{y}");
    println!("{z}");



    //functions  ->
    pub fn event(name:String){

        let name:String = String::from("Mishal");
        println!("{}",name);
    }

     let e = EventForKids {
        name : String::from("Mishal"),
        date : String::from("15.05.20204"),
        number_of_participents : 1000,
        place : String::from("Bhopal , India")

     };

     //ADD YOU enums


}


//struct  -> compiling many item in on eclass 
struct EventForKids{
    name:String,
    date:String,
    number_of_participents:u32,
    place:String,
}

//enums-> compiling errors in one class 

enum ErrorForEvent {
    NoEvent,
    CancleEvent,
    EventType
}