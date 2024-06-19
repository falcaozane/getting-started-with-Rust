fn main() {
    //println!("Hello, zane!");

    // let num:u8 = 5;
    // println!("the number is {}",num);

    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The updated value of x is: {x}");

    // // addition
    // let sum = 5 + 10;
    // println!("the sum is : {sum}");

    // // subtraction
    // let difference = 95.5 - 4.3;
    // println!("the difference is : {difference}");

    // // multiplication
    // let product = 4 * 30;
    // println!("the product is : {product}");

    // // division
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1
    // println!("the quotient of f64 is : {quotient}");
    // println!("the truncated of i32 is : {truncated}");

    // // remainder
    // let remainder = 43 % 5;
    // println!( "the remainder is : {remainder}");

    // let string_literal:&str = "Hello, world!"; // fixed length strings
    // println!("the string literal is : {string_literal}");

    // let mut string_literal2:String = String::from("Hello, Zane"); // dynamic length strings
    // string_literal2.push_str(" Falcao");
    // println!("the string literal is : {string_literal2}");

    // // tuple
    // let emp_info:(&str,u8) = ("Zane",50);
    // let emp_name = emp_info.0;
    // let emp_age = emp_info.1;
    // println!("the employee name is : {emp_name}");
    // println!("the employee age is : {emp_age}");

    // // tuple destructuring
    // let (employee_name,employee_age) = emp_info;
    // println!("the employee name is : {employee_name}");
    // println!("the employee age is : {employee_age}");

    
    
    struct FactorialResult {
        valid: bool,
        value: i32,
    }
    fn factorial(n: i32) -> FactorialResult {
        // Assumes n is non-negative.
        fn f(n: i32) -> i32 {
            if n == 0 { 1 } else { n * f(n - 1) }
        }
        let valid = n >= 0;
        let value = if valid { f(n) } else { n };
        FactorialResult {
            valid,
            value,
        }
    }
    
    let result = factorial(5);
    if result.valid {
        println!("The factorial of 5 is {}", result.value);
    } else {
        println!("Cannot calculate factorial of a negative number.");
    }
    let result = factorial(-5);
    if result.valid {
        println!("The factorial of -5 is {}", result.value);
    } else {
        println!("Cannot calculate factorial of a negative number.");
    }
    
}
