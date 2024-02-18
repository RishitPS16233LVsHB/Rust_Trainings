
fn main(){

    println!("enter the total elements to be entered in the array: ");
    let total_elements = get_int();

    println!("enter the total number of times you want to rotate array from left to right: ");
    let times = get_int();

    let mut my_array:Vec<i32> = Vec::new();

    for i in 0..total_elements{
        println!("enter the {i}th element of array: ");
        my_array.push(get_int());
    }

    println!("array before rotation: {:?} ",my_array);
    rotate_array(&mut my_array,times);
    println!("array after rotating {times}times: {:?}",my_array);
}

fn get_int() -> i32{
    let mut string_buffer:String = String::new();
    std::io::stdin().read_line(&mut string_buffer).expect("error at reading line");
    let int_number:i32 = string_buffer.trim().parse::<i32>().unwrap_or_else(|_| -1);
    int_number
}

fn rotate_array(array:&mut Vec<i32>, times:i32){

    for _i in 0..times {
        let last_element:i32 = array.last().expect("error getting the last element").clone();
        array.remove(array.len() - 1);
        array.insert(0,last_element);
    }
}