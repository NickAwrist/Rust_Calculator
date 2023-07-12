use std::borrow::Borrow;
use std::io;

fn input() -> String{
    let mut input = String::new();
    println!("Enter an equation");
    io::stdin().read_line(&mut input).expect("failed to readline");
    return input;
}

fn parenthesis_check(equation :&String) -> bool{

    let mut stack:Vec<String> = Vec::new();

    for i in equation.chars() {

        if i == '(' {
            stack.push(i.to_string());
        }else if i == ')' {
            if stack.pop().expect("Error") != "("  {
                return false;
            }
        }
    }

    return stack.len() == 0;
}

// Is our symbol addition, or subtraction
fn is_add_or_sub(symbol :&String) -> bool {
    return symbol == "+" || symbol == "-";
}

// Is our symbol multiplication, division, or modulo
fn is_multi_or_div(symbol : &String) -> bool{
    return symbol == "*" || symbol == "/" || symbol == "%";
}

// Is our symbol a symbol at all
fn is_symbol(symbol : &String) -> bool {
    return is_multi_or_div(&symbol) || is_add_or_sub(&symbol) || symbol == "^";
}

// Returns precedence level for different operators
fn precedence_level(symbol :&String) -> u8 {

    if is_add_or_sub(&symbol){
        return 1;

    }else if is_multi_or_div(&symbol){
        return 2;

    }else if symbol == "^" {
        return 3;
    }

    return 4;
}

// Pops all operators from the stack
fn pop_operators(stack: &mut Vec<String>, postfix: &mut Vec<String>, symbol:&String) {

    while !stack.is_empty() && stack[stack.len()-1] != "("  && precedence_level(symbol) <= precedence_level(&stack[stack.len()-1]) {
        postfix.push(stack.pop().expect("Error, not a string in stack")+" ");
    }
}

// Pops all values from the stack in between parenthesis
fn pop_parenthesis(stack: &mut Vec<String>, postfix: &mut Vec<String>) {

    while !stack.is_empty() && stack[stack.len()-1] != "(" {
        postfix.push(stack.pop().expect("Error, not a string in stack")+" ");
    }
    if !stack.is_empty() && stack[stack.len()-1] == "(" {
        stack.pop();
    }
}

// Infix expression to postfix expression
fn infix_to_postfix(equation :&String) -> Vec<String> {

    // Postfix and stack vectors
    let mut postfix:Vec<String> = Vec::new();
    let mut stack:Vec<String> = Vec::new();

    // Character array from equation string
    let chars = equation.chars().collect::<Vec<char>>();

    // The current index we are at in char array
    let mut current_index:usize = 0;

    // While we are in the char array
    while current_index < chars.len() {

        // Get current char
        let mut i:char = chars[current_index];

        // Get the symbol on the top of the stack (X is default value)
        let mut top_stack_symbol = &"X".to_string();

        if !stack.is_empty(){
            top_stack_symbol = stack.last().expect("Not a string");
        }

        // If our char is a digit, get all the following digits connected to it
        if i.borrow().is_digit(10){

            // Current digit we are at
            let mut current_number:String = i.to_string();

            // While our current char is a digit, add it to current_number
            while i.borrow().is_digit(10) {

                // Increment our current char and assign it to i
                current_index = current_index.borrow() + 1;
                if current_index >= chars.len() {
                    break;
                }
                i = chars[current_index];

                // If we incremented to not a digit, break
                if !i.is_digit(10) {
                    break;
                }

                // Add i to current_number
                current_number.push_str(&*i.to_string());
            }

            // Push current_number to the stack and end while loop iteration
            postfix.push(current_number+" ");

            continue;
        }

        // If we have an open parenthesis, push it to the stack
        if i == '(' {
            stack.push(i.to_string());

        // If our current value is an operator with higher precedence, add all operators to postfix and i to the stack
        }else if !stack.is_empty() && precedence_level(&i.to_string()) <= precedence_level(&top_stack_symbol) {
            pop_operators(&mut stack, &mut postfix, &i.to_string());
            stack.push(i.to_string());

        // If we are at a closed parenthesis, add everything to the postfix up until an open parenthesis
        }else if !stack.is_empty() && i == ')' {
            pop_parenthesis(&mut stack, &mut postfix);

        // If i is an operation, push it to the stack
        } else if is_add_or_sub(&i.to_string()) || is_multi_or_div(&i.to_string()) || i == '^' {
            stack.push(i.to_string());

        }

        // Increment our current position in the char array
        current_index = current_index + 1;
    }

    // Empty the rest of the stack into postfix
    while !stack.is_empty() {
        postfix.push(stack.pop().expect("Error: Pop failed\n")+" ");
    }

    return postfix;
}

// Recursive exponent function
fn pow(base :&f64, exponent :i64, positive :bool) -> f64 {
    return if positive {
        if exponent <= 0 {
            1.0
        } else {
            base * pow(&base, exponent - 1, positive)
        }
    } else {
        1.0 / pow(&base, (-1)*exponent, !positive)
    }
}

// Evaluates the postfix
fn evaluate_postfix(postfix :&String) -> f64 {

    // Stack vector
    let mut stack:Vec<String> = Vec::new();

    // Final value to return
    let final_value:f64;

    // Char array from postfix string
    let chars = postfix.chars().collect::<Vec<char>>();

    // Current index we are at in char array
    let mut current_index:usize = 0;

    // While we are in the char array
    while current_index < chars.len() {
        let mut i:char = chars[current_index];

        if i.borrow().is_digit(10){

            // Current digit we are at
            let mut current_number:String = i.to_string();

            // While our current char is a digit, add it to current_number
            while i.borrow().is_digit(10) {

                // Increment our current char and assign it to i
                current_index = current_index.borrow() + 1;
                if current_index >= chars.len() {
                    break;
                }
                i = chars[current_index];

                // If we incremented to not a digit, break
                if !i.is_digit(10) {
                    break;
                }

                // Add i to current_number
                current_number.push_str(&*i.to_string());
            }

            // Push the new number into the stack
            stack.push(current_number.to_string());
            continue;
        }

        // If i is an operator
        if is_symbol(&i.to_string()) {

            // Parse the top two values in stack into floats
            let val1:f64 = stack.pop().expect("Error: Pop failed").trim().parse::<f64>().expect("Error: Parse failed");
            let mut val2:f64 = stack.pop().expect("Error: Pop failed").trim().parse::<f64>().expect("Error: Parse failed");

            // Use operators on the two values
            if i == '*' {
                val2 = val2 * val1;
            }else if i == '/' {
                val2 = val2 / val1;
            }else if i == '+' {
                val2 = val2 + val1;
            }
            else if i == '%' {
                val2 = val2 % val1;
            }else if i == '-' {
                val2 = val2 - val1;
            }else if i == '^' {
                val2 = pow(&val2, val1 as i64, val1 > 0.0);
            }

            // Push the new value into the stack
            stack.push(val2.to_string());
        }

        // Increment our position in the char array
        current_index = current_index + 1;
    }

    // Return last value in the stack
    final_value = stack.pop().expect("Error: Pop failed").trim().parse::<f64>().expect("Error: Parse failed");
    return final_value;
}


fn main() {

    println!("Welcome to a Rust implementation of a calculator!");
    println!("How to use (a and b are arbitrary values):
    a*b for multiplication
    a/b for division
    a%b for modulus
    a+b for addition
    a-b for subtraction
    a^b for exponent
    (0-a) for a negative number");

    // Get equation input
    let equation = input();

    // If the parenthesis are balanced, evaluate the equation
    if parenthesis_check(&equation) {
        // Get postfix
        let postfix:Vec<String> = infix_to_postfix(&equation);

        // Turn postfix vector into a string
        let mut postfix_string:String = String::new();
        for i in postfix {
            postfix_string = postfix_string + &*i.to_string();
        }

        // Evaluate the postfix
        let final_value:f64 = evaluate_postfix(&postfix_string);

        println!("{} = {}", equation.trim(), final_value);


    } else {
        eprintln!("Equation has invalid parenthesis!!!");
        return;
    }
}