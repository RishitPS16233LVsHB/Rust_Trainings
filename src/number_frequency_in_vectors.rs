use std::collections::HashMap;

fn main(){
    let array_of_numbers:Vec<i32> = vec!(2,5,23,3,56,6,56,4,3,43,5,6,4,3,2,2,56,4,6,1);
    let frequencies = get_frequency_of_numbers(&array_of_numbers);

    for (num,fi) in &frequencies{
        println!("number {num} has {fi} occurences");
    }
}

fn get_frequency_of_numbers(array_of_numbers:&Vec<i32>) -> HashMap<i32,i32> {
    let mut _frequencies:HashMap<i32,i32>  = HashMap::new();

    for i in array_of_numbers.iter(){
        let freq = _frequencies.entry(*i).or_insert(0);
        *freq += 1;
    }
    _frequencies
}