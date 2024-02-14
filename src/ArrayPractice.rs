use std::io::{Read, Write};

fn main(){
    let mut array_of_numbers:[[i32;5];5] = [[0;5];5];

    println!("{:?}",array_of_numbers);
    let mut i = 0;
    let mut j = 0;
    let total_rows = 5;
    let total_cols = 5;

    while i < total_rows {
        while j < total_cols {
            let mut int_value:i32 = 0;
            let mut string_buffer:String = String::new();

            println!("enter number for matrix {i} {j} ");
            std::io::stdin().read_line(&mut string_buffer).expect("error while reading input");
            int_value = string_buffer.trim().parse().expect("failed to convert string to int");

            array_of_numbers[i][j] = int_value;
            j += 1;
        }
        j = 0;
        i += 1;
    }

    i = 0;
    j = 0;

    println!("the matrix is : ");
    while i < total_rows {
        while j < total_cols {
            print!("{}",array_of_numbers[i][j]);
            j += 1;
        }
        j = 0;
        print!("\n");
        i += 1;
    }
}

