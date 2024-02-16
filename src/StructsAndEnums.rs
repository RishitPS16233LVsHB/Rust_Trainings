#[derive(Debug)]
struct Person {
    id:i32,
    name:String,
    age:i32,
    salary:f32,
}
fn main() {

    const MY_CONST:i32 = 23;


    // full initialization of a struct
    // this is the only way to init a struct
    let mut person: Person = Person{
        id: 1,
        name: String::from("Rishit"),
        age: 32,
        salary: 12000.0
    };

    // fields can be accessed using the classic dot operator
    println!("{:?}",person);
    println!(" person id: {}\n person name : {}\n person age: {}\n person salary: {}",
             person.id,
             person.name,
             person.age,
             person.salary);

    // fields can be assigned using the dot operator too
    person.id = 2;
    person.name = "Hetvi Bodhanwala".to_string();
    person.age = 22;
    person.salary = 23000.0;

    println!("{:?}", person);

    // also remeber if you are try to assign the value of a data member in a struct its owner ship
    // will be transfered from the struct variable to the assigning variable
    // meaning that field will no longer be accessible from the structs
    // if the field was heap allocated like strings, dates, other structs, vectors etc.

    //let name = person.name;
    //println!("the name of a person is : {name}");
    //println!("trying to print name from the person: {}",person.name);

    // we can copy init some or all properties of structs using ..<struct variable name>

    let person2:Person = Person {
      ..person
    };

    println!("the person 2 is {:?}",person2);
    // if i wanted to change the name and the id but age and salary same we can do the same using the .. operation

    let person3:Person = Person {
        id: 3,
        name: "Thanuja HR".to_string(),
        ..person2
    };

    println!("the person 3 is {:?}",person3);


    // tuple structs: we can define struct from the tuple but the fields are not named only types are passed
    struct PersonFromTuple(i32,String,i32,i32);
    let tuple_person4:PersonFromTuple = PersonFromTuple(1,"Thanuja HR".to_string(),23,35000);

    println!(" the id of person is {}\n the name of the person is {}\n the age of the person is {}\n the salary of the person is {}",
             tuple_person4.0,
             tuple_person4.1,
             tuple_person4.2,
             tuple_person4.3);
}