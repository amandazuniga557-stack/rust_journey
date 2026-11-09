use std::io;

fn main() {
   println!("--- Rust Calculator ---");

   // Get first number
   println!("Enter your first number:");
   let mut num1 = String::new();
   io::stdin()
       .read_line(&mut num1)
       .expect("Failed to read line");
    let num1: i32 = num1.trim().parse().expect("Please type a valid number!");  
    
    // Get operator
    println!("Enter an operator (+, -, *, /):");
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");
    let operator = operator.trim();    

    //Get second number
    println!("Enter your second number:");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("F(ailed to read line");
    let num2: i32 = num2.trim().parse().expect("Please type a valid number!");


    // Match the operator to preform the right math 

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _   => {
            println!("Unkown operator!");
            return;
        }
    };        

    println!("Result: {} {} {} = {}",num1, operator, num2, result);  

}