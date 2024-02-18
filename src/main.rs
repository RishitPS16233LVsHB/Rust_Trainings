mod ArrayPractice;
mod Beginners;
mod StructsAndEnums;
mod StructsWithImplementationsAndEnums;
mod ArrayOperations;

fn main(){

    let my_string:&str = "";
    println!("my string was {my_string}");

    let reversed_string:String = reverse_string(my_string);
    println!("my reversed string is {reversed_string}");

    let tuple_employee:(i32,String) = (1,String::from("rishit"));
    println!("the struct is {:?} the id is {} and name is {}",tuple_employee,tuple_employee.0,tuple_employee.1);

    let mut id:i32 = 0;
    let mut name:String = String::new();
    println!("the id is {id} and the name is {name}");
    (id,name) = tuple_employee;
    println!("the id is {id} and the name is {name}");
}

fn reverse_string(string:&str) -> String{

    if string.is_empty()  { return String::from(""); }

    let s:String = String::from(string);
    let mut reversed_string:String = String::from("");
    let mut char_indices = s.char_indices();

    while let Some((_,c)) = char_indices.next_back(){
        reversed_string.push(c);
    }

    reversed_string
}