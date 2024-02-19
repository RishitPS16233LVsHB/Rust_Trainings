
struct Employee {
    id: u32,
    name:Option<String>,
    age: u8,
    salary: u32
}

impl Employee {
    fn describe_employee(&self){
        println!();
        println!("Employee name : {}", match &self.name {
            Some(T) => T,
            _ => "N/A"
        });
        println!("Employee id: {}",self.id);
        println!("Employee age: {}",self.age);
        println!("Employee salary: {}",self.salary);
        println!();
    }
}

fn main(){
    let mut employee_database:Vec<Employee> = Vec::new();
    insert_into_employee_database(&mut employee_database);

    let defect_employee: Employee = Employee{
        id: 0,
        name: None,
        age: 0,
        salary: 0,
    };
    employee_database.push(defect_employee);

    show_employees(&employee_database);

    println!("who would you like to search? ");
    let name:String = get_string();

    let target_employee = find_by_name(&employee_database,&name);
    match target_employee {
        Ok(_e) => println!("found him"),
        Err(e) => println!("cant find him with the message {e}")
    }
}
fn get_int() -> i32{
    let mut buffer:String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("error at reading buffer");
    buffer.trim().parse::<i32>().expect("can't convert this string to int")
}
fn get_string() -> String {
    let mut buffer:String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("error at reading buffer");
    buffer.trim().to_string()
}
fn get_uint() ->u32{
    let mut buffer:String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("error at reading buffer");
    buffer.trim().parse::<u32>().expect("can't convert this string to uint")
}
fn get_ubyte() -> u8{
    let mut buffer:String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("error at reading buffer");
    buffer.trim().parse::<u8>().expect("can't convert this string to uint")
}
fn insert_into_employee_database(employee_database:&mut Vec<Employee>){
    println!("enter total new employees to be added to database: ");
    let total_employees: i32 = get_int();

    for _i in 0..total_employees {
        println!("enter the id of the employee: ");
        let id:u32 = get_uint();
        println!("enter the name of the employee: ");
        let name:String = get_string();
        println!("enter the age of the employee: ");
        let age:u8 = get_ubyte();
        println!("enter the salary of the employee: ");
        let salary:u32 = get_uint();


        let employee:Employee = Employee{
            id:id,
            name:Some(name),
            age:age,
            salary:salary
        };
        employee_database.push(employee);
    }

}
fn show_employees(employee_database:&Vec<Employee>){
    for e in employee_database{
        e.describe_employee();
    }
}
fn find_by_name(employee_database:&Vec<Employee>,name:&String) -> Result<Employee,String>{
    for e in employee_database{
        if match &e.name {
            Some(T) => T,
            _ => "N/A"
        } == *name {
            let emp:Employee = Employee{
                id: e.id,
                name: e.name.clone(),
                age: e.age,
                salary: e.salary
            };
            return Ok(emp);
        }
    }
    Err("cant find that employee".to_string())
}