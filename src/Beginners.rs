fn main() {
    say_hello_world();

    println!("enter a number to check if the number is odd or even? ");
    let number1: i32 = get_int();

    println!("is the {number1} odd or even? {}", check_odd_even(&number1));

    println!("enter a number for getting factorial of it: ");
    let number1: i32 = get_int();
    let factorial:i128 = factorial(&number1);

    println!("the factorial of the number is: {factorial}");
}

fn say_hello_world() {
    println!("hello world");
}

fn check_odd_even(number: &i32) -> bool {
    number % 2 == 0
}

// for getting integer inputs
fn get_int() -> i32 {
    let mut input_buffer: String = String::new();
    std::io::stdin()
        .read_line(&mut input_buffer)
        .expect("error at reading line");
    let int_val = match input_buffer.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            return 0;
        }
    };

    return int_val;
}


// recursion but felt very alien using it
fn factorial(number: &i32) -> i128 {
    if *number <= 0 {
        return 1;
    }
    return (*number as i128) * factorial(&(*number - 1));
}
