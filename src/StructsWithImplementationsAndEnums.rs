// structs is a user defined data structure but to that we can add functionality too
// first we define a struct employee



// type aliasing to c/c++
type int = i32;
type string = String;
type float = f32;
type double = f64;
type long = i64;
type long_long = i128;
type short= i16;
type byte = i8;
type ubyte = u8;
type ushort = u16;
type uint = u32;
type ulong = u64;
type ulonglong = u128;


struct Employee {
    id: int,
    name: string,
    age: int,
    salary: float,
    department: string,
}
// like in object oriented programming methods can be associated to rust's structs using the impl key word
impl Employee{
    // we are implementing struct based method both static and non static over here

    // non static or member methods can be defined as
    // notice &self here self is the reference to "this" object or struct variable
    // here we are passing immutable reference to this function
    fn say_your_name(&self) {
        println!("hello my name is {}",self.name);
        println!("I work in {}",self.department);
        println!("my salary is {}",self.salary);
    }

    // we can make static method by not using the first parameter as self
    // remember using first parameter in the method makes the method non static and static otherwise
    // implementing a method inside an impl <struct_name> makes function a member method like in oop

    // static method and also a constructor for our employee struct
    fn create_new_instance() -> Self {
        let _new:Self =  Employee {
            id : 0,
            name : "".to_string(),
            department : "".to_string(),
            age: -1,
            salary: -1.0,
        };
        _new
    }

    // parameterized static constructor
    fn init_employee(id:int,name:string,age:int,salary:float,department:string) -> Self {
        let _new:Self = Self{
            id: id,
            name:name,
            salary:salary,
            department:department,
            age:age,
        };
        _new
    }

    // we can update the state of the struct variable using the mutable reference to self
    fn increase_salary(&mut self,increase_by:float){
        self.salary += increase_by;
    }

    // create a copy constructor
    fn copy_constructor(this_instance:&Employee) -> Employee{
        let _new:Employee = Employee{
            id: this_instance.id,
            name: this_instance.name.clone(),
            age: this_instance.age,
            salary: this_instance.salary,
            department: this_instance.department.clone()
        };
        _new
    }

}

// let us test our code here
fn main(){

    // will create a default instance of struct employee
    let employee1: Employee = Employee::create_new_instance();
    employee1.say_your_name();

    // use of parameterized constructor
    let mut employee2: Employee = Employee::init_employee(1,"Rishit".to_string(),22,12000.0,"SOFTWARE_DEVELOPMENT".to_string());
    employee2.say_your_name();

    //copy constructor usage on employee2 to employee3
    let employee3: Employee = Employee::copy_constructor(&employee2);
    employee3.say_your_name();

    // incrementing salary of employee2 by 10000
    employee2.increase_salary(10000.0);
    employee2.say_your_name();


}