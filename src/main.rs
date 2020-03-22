
use std::io;
use std::collections::hash_map::{HashMap};

const OPERATOR_PLUS: &char = &'+';
const OPERATOR_MINUS: &char = &'-';
const OPERATOR_MULTIPLY: &char = &'*';
const OPERATOR_DIVIDE: &char = &'/';

fn tokenize(expression: String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut number = String::with_capacity(10);
    for c in expression.chars() {
        if c.is_whitespace() {
            continue;
        } else if c.is_numeric() {
            number.push(c);
        } else if c == '+' || c == '-'|| c == '/' || c == '*' || c == '(' || c == ')' {
            if number.len() > 0 {
                tokens.push(number.clone());
            }
            number.clear();
            tokens.push(c.to_string());
        } else {
            panic!("Unexpected symbol '{}'", c);
        }
    }
    if number.len() > 0 {
        tokens.push(number);
    }
    tokens
}

fn main() {
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).unwrap();
    println!("Input expression: {}", expression);

    let tokens = tokenize(expression);
    for token in &tokens {
        println!("{}", token);
    }

    let mut operator_priority_map = HashMap::new();
    operator_priority_map.insert(OPERATOR_PLUS, 0);
    operator_priority_map.insert(OPERATOR_MINUS, 0);
    operator_priority_map.insert(OPERATOR_MULTIPLY, 1);
    operator_priority_map.insert(OPERATOR_DIVIDE, 1);

    let mut reversed_polish: Vec<String> = Vec::new();
    let mut stack: Vec<char> = Vec::new();
    for token in &tokens {
        for c in token.chars() {
            if c.is_numeric() {
                reversed_polish.push(token.clone());
                break;
            } else if c == '(' {
                stack.push(c);
                break;
            } else if c == ')' {
                loop {
                    let result = stack.pop();
                    match result {
                        None => panic!("Can't find matching brace"),
                        Some(value) => {
                            if value == '(' {
                                break;
                            } else {
                                reversed_polish.push(value.to_string());
                            }
                        }
                    }
                }
                break;
            } else {
                let result = operator_priority_map.get(&c);
                match result {
                    None => panic!("Wrong expression, unexpected binary operation '{}'", c),
                    Some(priority) => {
                        loop {
                            match stack.last() {
                                None => break,
                                Some(value) => {
                                    let stack_top_result = operator_priority_map.get(value);
                                    match stack_top_result {
                                        None => break,
                                        //panic!("Something strange happened, current value {}", c),
                                        Some(stack_top_priority) => {
                                            if stack_top_priority >= priority {
                                                reversed_polish.push(value.to_string());
                                                stack.pop();
                                            } else {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                stack.push(c);
                break;
            }
        }
    }

    loop {
        match stack.pop() {
            None => break,
            Some(value) => {
                reversed_polish.push(value.to_string());
            }
        }
    }

    println!("Reversed polish");
    for item in &reversed_polish {
        print!("{} ", item);
    }
    println!("");

    let mut calculation_stack: Vec<f32> = Vec::new();
    for item in &reversed_polish {
        let first_symbol = item.chars().next().unwrap();
        if first_symbol.is_numeric() {
            let number: f32 = item.parse().unwrap();
            calculation_stack.push(number);
        } else {
            let right = calculation_stack.pop().unwrap();
            let left = calculation_stack.pop().unwrap();
            let result;
            match &first_symbol {
                OPERATOR_PLUS => result = left + right,
                OPERATOR_MINUS => result = left - right,
                OPERATOR_MULTIPLY => result = left * right,
                OPERATOR_DIVIDE => result = left / right,
                _ => result = 0.0,
            }
            calculation_stack.push(result);
        }
    }

    let result = calculation_stack.pop().unwrap();
    println!("Result = {}", result);
}