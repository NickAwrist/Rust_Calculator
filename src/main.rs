use std::io;

fn input() -> String{
    let mut input = String::new();
    println!("Enter an equation");
    io::stdin().read_line(&mut input).expect("Error: Input failed");
    return input;
}

// Checks if equation parenthesis are balanced
fn parenthesis_check(equation :&String) -> bool{

    let mut stack:Vec<String> = Vec::new();

    for i in equation.chars() {

        if i == '(' {
            stack.push(i.to_string());
        }else if i == ')' {
            let pop_val:Option<String> = stack.pop();
            if pop_val == None || pop_val.clone().expect("Error: Stack failed to pop") != "("{
                return false;
            }
        }
    }

    return stack.len() == 0;
}

// Is our symbol addition, or subtraction
fn is_add_or_sub(symbol :u8) -> bool {
    return symbol == b'+' || symbol == b'-';
}

// Is our symbol multiplication, division, or modulo
fn is_multi_or_div(symbol :u8) -> bool{
    return symbol == b'*' || symbol == b'/' || symbol == b'%';
}

// Is our symbol a symbol at all
fn is_symbol(symbol :u8) -> bool {
    return is_multi_or_div(symbol) || is_add_or_sub(symbol) || symbol == b'^' || symbol == b'!';
}

// Returns precedence level for different operators
fn precedence_level(symbol :u8) -> u8 {

    if is_add_or_sub(symbol){
        return 1;

    }else if is_multi_or_div(symbol){
        return 2;

    }else if symbol == b'^' {
        return 3;

    }else if symbol == b'!' {
        return 4;
    }

    // Default
    return 5;
}

// Pops all operators from the stack
fn pop_operators(stack: &mut Vec<u8>, postfix: &mut Vec<u8>, symbol:u8) {

    while !stack.is_empty() && stack[stack.len()-1] != b'('  && precedence_level(symbol) <= precedence_level(stack[stack.len()-1]) {
        postfix.push(stack.pop().expect("Error: Stack failed to pop"));
        postfix.push(b' ');
    }
}

// Pops all values from the stack in between parenthesis
fn pop_parenthesis(stack: &mut Vec<u8>, postfix: &mut Vec<u8>) {

    while !stack.is_empty() && stack[stack.len()-1] != b'(' {
        postfix.push(stack.pop().unwrap());
        postfix.push(b' ');
    }
    if !stack.is_empty() && stack[stack.len()-1] == b'(' {
        stack.pop();
    }
}

// Infix expression to postfix expression
fn infix_to_postfix(equation :&String) -> Vec<u8> {

    // Postfix and stack vectors
    let mut postfix:Vec<u8> = Vec::new();
    let mut stack:Vec<u8> = Vec::new();

    let equation_vec:Vec<u8> = equation.clone().into_bytes();

    // The current index we are at in char array
    let mut current_index:usize = 0;


    let mut i:u8;

    let mut last_was_symbol:bool = false;

    // While we are in the char array
    while current_index < equation_vec.len() {

        // Get current ascii
        i = equation_vec[current_index];

        // Get the symbol on the top of the stack (x is default value)
        let mut top_stack_symbol:u8 = b'x';

        if !stack.is_empty(){
            top_stack_symbol = stack[stack.len() - 1];
        }

        if last_was_symbol && i == b'-' {
            postfix.push(b'&');
            last_was_symbol = false;

            current_index = current_index + 1;
            if current_index >= equation_vec.len() {
                break;
            }
            continue;
        }

        // If our char is a digit, get all the following digits connected to it
        if i.is_ascii_digit(){

            postfix.push(i);
            current_index = current_index + 1;
            if current_index >= equation_vec.len() {
                break;
            }
            i = equation_vec[current_index];

            // While our current char is a digit, add it to the postfix without spaces
            while i.is_ascii_digit() {

                postfix.push(i);

                // Increment our current char and assign it to i
                current_index = current_index + 1;
                if current_index >= equation_vec.len() {
                    break;
                }
                i = equation_vec[current_index];

            }

            // Push a space to separate the number
            postfix.push(b' ');
            last_was_symbol = false;
            continue;
        }

        // If we have an open parenthesis, push it to the stack
        if i == b'(' {
            stack.push(i);

        // If our current value is an operator with higher precedence, add all operators to postfix and i to the stack
        }else if !stack.is_empty() && precedence_level(i) <= precedence_level(top_stack_symbol) {
            pop_operators(&mut stack, &mut postfix, i);
            stack.push(i);
            last_was_symbol = true;

        // If we are at a closed parenthesis, add everything to the postfix up until an open parenthesis
        }else if !stack.is_empty() && i == b')' {
            pop_parenthesis(&mut stack, &mut postfix);

        // If i is an operation, push it to the stack
        }else if is_symbol(i) {
            stack.push(i);
            last_was_symbol = true;

        }else if i == b' ' || i == b'\n' || i == b'\t' {
            current_index = current_index + 1;
            continue;
        }else {
            // Error if any invalid symbols
            println!("ERROR: Invalid Symbol {}", i as char);
            return vec![0];
        }

        // Increment our current position in the char array
        current_index = current_index + 1;
    }

    // Empty the rest of the stack into postfix
    while !stack.is_empty() {
        postfix.push(stack.pop().expect("Error, not a character in stack"));
        postfix.push(b' ');
    }

    return postfix;
}

// Recursive exponent function
fn pow(base :f64, exponent :i64, positive :bool) -> f64 {
    return if positive {
        if exponent <= 0 {
            1.0
        } else {
            base * pow(base, exponent - 1, positive)
        }
    } else {
        1.0 / pow(base, (-1)*exponent, true)
    }
}

// Recursive factorial function
fn factorial(base :f64) -> f64{
    return if base == 0.0 {
        1.0
    } else {
        base * factorial(base - 1.0)
    }
}

// Evaluates the postfix
fn evaluate_postfix(postfix :Vec<u8>) -> f64 {

    println!();
    for i in &postfix {
        print!("{}", *i as char);
    }
    println!();

    // Check if invalid symbols
    if postfix.is_empty() {
        return 0.0;
    }

    // Stack vector
    let mut stack:Vec<f64> = Vec::new();

    // Final value to return
    let final_value:f64;

    // Current index we are at in char array
    let mut current_index:usize = 0;

    let mut negative_multiply:i8 = 1;

    // While we are in the char array
    while current_index < postfix.len() {

        // Grab the current char ASCII
        let mut i: u8 = postfix[current_index];

        if i == b'&' {
            negative_multiply = -1;
            current_index = current_index+1;
            if current_index >= postfix.len() {
                break;
            }

            continue;
        }

        // If i is a digit, find out what number it is
        if i.is_ascii_digit() {
            // Default 0
            let mut value: f64 = 0.0;

            // Current digit we are at (0-9)
            let mut current_number: u8 = i - b'0';

            // Add current digit to value with its corresponding base (12 = 1*10 + 2*1)
            value = value * 10 as f64 + current_number as f64;

            // Increment i
            current_index = current_index + 1;
            if current_index >= postfix.len() {
                break;
            }
            i = postfix[current_index];

            // While i is a digit
            while i.is_ascii_digit() {

                // Current digit we are at (0-9)
                current_number = i - b'0';

                // Add current digit to value with its corresponding base (12 = 1*10 + 2*1)
                value = value * 10 as f64 + current_number as f64;

                // Increment i
                current_index = current_index + 1;
                if current_index >= postfix.len() {
                    break;
                }
                i = postfix[current_index];
            }

            // Push value to the stack
            stack.push(value * negative_multiply as f64);
            negative_multiply = 1;
            continue;
        }
        // If i is an operator
        if is_symbol(i) {


            // Default values for val1 and val2
            let val1:f64;
            let mut val2:f64 = 0.0;

            // Grab the top two values from stack
            let first_pop:Option<f64> = stack.pop();
            let second_pop:Option<f64> = stack.pop();

            // If the first pop is none, print an error and return 0.0
            if first_pop == None {
                println!("Error: No number found\n");
                return 0.0;
            }
            else {
                // Assign val1 to its respective value
                val1 = first_pop.expect("Error: Stack failed to pop");

                // If the second pop is not None, assign its respective value
                if second_pop != None {
                    val2 = second_pop.expect("Error: Stack failed to pop");
                }
            }


            // Use operators on the two values
            match i {
                b'*'=>{
                    val2 = (val2) * (val1);
                },
                b'/'=>{
                    val2 = (val2) / (val1);
                },
                b'%'=>{
                    val2 = (val2) % (val1);
                },
                b'+'=>{
                    val2 = (val2) + (val1);
                },
                b'-'=>{
                    val2 = (val2) - (val1);
                },
                b'^'=>{
                    val2 = pow(val2, val1 as i64, val1 > 0.0);
                },
                b'!'=>{
                    stack.push(val2);
                    val2 = factorial(val1);
                },
                _ => println!("Error: Unknown operator in postfix")
            }

            // Push the new value into the stack
            stack.push(val2);
        }

        // Increment our position in the char array
        current_index = current_index + 1;
    }

    // Return last value in the stack
    final_value = stack.pop().expect("Error: Stack failed to pop");
    return final_value;
}


fn main() {

    // Get equation input
    let mut equation:String = String::new();


    while equation.trim().to_lowercase() != "Exit".trim().to_lowercase() {
        println!("Welcome to a Rust implementation of a calculator!");
        println!("How to use (a and b are arbitrary values):
        a*b for multiplication
        a/b for division
        a%b for modulus
        a+b for addition
        a-b for subtraction
        a^b for exponent
        a!  for factorial
        (-a) for negative (beta)
    Type 'Exit' to exit calculator");

        equation = input();

        if equation.trim().to_lowercase() == "Exit".trim().to_lowercase() {
            break;
        }

        // If the parenthesis are balanced, evaluate the equation
        if parenthesis_check(&equation) {
            // Get postfix
            let postfix: Vec<u8> = infix_to_postfix(&equation);

            // Evaluate the postfix
            let final_value:f64 = evaluate_postfix(postfix);

            println!("{} = {}\n", equation.trim(), final_value);


        } else {
            eprintln!("Equation has invalid parenthesis!!!");
        }
    }

    println!("Thank you!");
}