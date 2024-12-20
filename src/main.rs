fn main() {
    println!("Calculator App!");

    loop {
        println!("Hello, what would you like to do?");

        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("Any Value: Exit");

        let input = read_string().trim().to_string();

        print_type_of(&input);

        println!("Your Input: {input}");

        match input.as_str() {
            "1" => println!("1. Add"),

            "2" => println!("2. Subtract"),

            "3" => println!("3. Multiply"),

            "4" => println!("4. Divide"),

            _ => break,
        }

        println!("Enter First Value");

        let val1 = read_string().trim().to_string();

        println!("Enter Second Value");

        let val2 = read_string().trim().to_string();

        if check_if_int(&val1) && check_if_int(&val2) {
            println!("We got a Integer yo")
        } else {
            println!("Value not a Integer yo");
            panic!("Value is NOT a Integer.");
        }

        let mut end_result: f64 = 0.0;

        if input == "1" {
            println!("Adding {} and {}\n", val1, val2);
            end_result = add_values(val1, val2).expect("Failed for no fun");
        } else if input == "2" {
            println!("Subtracting {} and {}\n", val1, val2);
            end_result = subtract_values(val1, val2);
        } else if input == "3" {
            println!("Multiplying {} and {}\n", val1, val2);
            end_result = multiply_values(val1, val2);
        } else if input == "4" {
            println!("Dividing {} and {}\n", val1, val2);
            end_result = divide_values(val1, val2);
        }

        println!("End Result: {}\n", end_result);
    }

    println!("Exiting! Thank you for using Calculator!");
}

fn read_string() -> String {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Can't Read User Input");

    input
}

fn add_values(val1: String, val2: String) -> Result<f64, std::num::ParseIntError> {
    let new_val1 = val1.parse::<i32>()?;
    let new_val2 = val2.parse::<i32>()?;
    Ok((new_val1 + new_val2) as f64)
}

fn multiply_values(val1: String, val2: String) -> f64 {
    let result = val1.parse::<f64>().unwrap() * val2.parse::<f64>().unwrap();
    result
}

fn subtract_values(val1: String, val2: String) -> f64 {
    let result = val1.parse::<f64>().unwrap() - val2.parse::<f64>().unwrap();
    result
}

fn divide_values(val1: String, val2: String) -> f64 {
    let result = val1.parse::<f64>().unwrap() / val2.parse::<f64>().unwrap();
    result
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn check_if_int(string: &str) -> bool {
    string.parse::<f64>().is_ok()
}
