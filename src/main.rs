
use std::io;
use std::collections::hash_map::HashMap;

const OPERATOR_PLUS: u8 = b'+';
const OPERATOR_MINUS: u8 = b'-';
const OPERATOR_MULTIPLY: u8 = b'*';
const OPERATOR_DIVIDE: u8 = b'/';
const LEFT_BRACE: u8 = b'(';
const RIGHT_BRACE: u8 = b')';

type AsciiBytes = Vec<u8>;

fn is_operator(byte: &u8) -> bool {
    match *byte {
        OPERATOR_PLUS |
        OPERATOR_MINUS |
        OPERATOR_MULTIPLY |
        OPERATOR_DIVIDE => true,
        _ => false
    }
}

fn is_bracket(byte: &u8) -> bool {
    match *byte {
        LEFT_BRACE | RIGHT_BRACE => true,
        _ => false
    }
}

fn tokens_from_ascii(expression_ascii: &[u8]) -> Vec<AsciiBytes> {
    let mut tokens_ascii: Vec<AsciiBytes> = Vec::new();
    let mut number: AsciiBytes = Vec::new();

    for &byte in expression_ascii {
        if byte.is_ascii_whitespace() {
            continue;
        } else if byte.is_ascii_digit() {
            number.push(byte);
        } else if is_operator(&byte) || is_bracket(&byte) {
            if number.len() > 0 {
                tokens_ascii.push(number.clone());
                number.clear();
            }
            number.push(byte);
            tokens_ascii.push(number.clone());
            number.clear();
        } else {
            panic!("Unexpected symbol {}", byte as char);
        }
    }

    if number.len() > 0 {
        tokens_ascii.push(number.clone());
        number.clear();
    }

    tokens_ascii
}

fn reversed_polish_from_tokens_ascii(tokens_ascii: &Vec<AsciiBytes>) -> Vec<AsciiBytes> {
    let mut operator_priority_map = HashMap::new();
    operator_priority_map.insert(OPERATOR_PLUS, 0);
    operator_priority_map.insert(OPERATOR_MINUS, 0);
    operator_priority_map.insert(OPERATOR_MULTIPLY, 1);
    operator_priority_map.insert(OPERATOR_DIVIDE, 1);

    let mut reversed_polish: Vec<AsciiBytes> = Vec::new();
    let mut stack: Vec<AsciiBytes> = Vec::new();

    for token in tokens_ascii.iter() {
        for &byte in token.iter() {
            if byte.is_ascii_digit() {
                reversed_polish.push(token.clone());
                break;
            } else if byte == LEFT_BRACE {
                stack.push(token.clone());
                break;
            } else if byte == RIGHT_BRACE {
                loop {
                    match stack.pop() {
                        None => panic!("Can't find matching brace"),
                        Some(value) => {
                            if *value.get(0).unwrap() == LEFT_BRACE {
                                break;
                            } else {
                                reversed_polish.push(value);
                            }
                        }
                    }
                }
                break;
            } else {
                match operator_priority_map.get(&byte) {
                    None => panic!("Wrong expression, unexpected binary operation '{}'", byte),
                    Some(priority) => {
                        loop {
                            match stack.last() {
                                None => break,
                                Some(operation_top_stack) => {
                                    match operator_priority_map.get(operation_top_stack.get(0).unwrap()) {
                                        None => break,
                                        Some(priority_top_stack) => {
                                            if priority_top_stack >= priority {
                                                reversed_polish.push(operation_top_stack.clone());
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
                stack.push(token.clone());
                break;
            }
        }
    }

    loop {
        match stack.pop() {
            None => break,
            Some(value) => {
                reversed_polish.push(value);
            }
        }
    }

    reversed_polish
}

fn eval_reversed_polish(reversed_polish_tokens: &Vec<AsciiBytes>) -> f32 {
    let mut eval_stack: Vec<f32> = Vec::new();

    for token in reversed_polish_tokens.iter() {
        let &first_byte = token.get(0).unwrap();
        if first_byte.is_ascii_digit() {
            let token_utf8 = String::from_utf8(token.clone()).unwrap();
            let number: f32 = token_utf8.parse().unwrap();
            eval_stack.push(number);
        } else {
            let right = eval_stack.pop().unwrap();
            let left = eval_stack.pop().unwrap();
            let result;
            match first_byte {
                OPERATOR_PLUS => result = left + right,
                OPERATOR_MINUS => result = left - right,
                OPERATOR_MULTIPLY => result = left * right,
                OPERATOR_DIVIDE => result = left / right,
                _ => result = 0.0,
            }
            eval_stack.push(result);
        }
    }

    eval_stack.pop().unwrap()
}

fn main() {
    let mut expression = String::new();
    println!("Please, enter an expression to calculate");
    io::stdin().read_line(&mut expression).unwrap();
    print!("Input expression: {}", expression);

    let tokens_ascii = tokens_from_ascii(expression.as_bytes());

    println!("Tokenization:");
    for (i, token) in tokens_ascii.iter().enumerate() {
        let token = String::from_utf8(token.clone()).unwrap();
        println!("Current token {} - '{}'", i, token);
    }
    println!("");

    let reversed_polish_tokens = reversed_polish_from_tokens_ascii(&tokens_ascii);

    println!("Reversed polish:");
    for (i, token) in reversed_polish_tokens.iter().enumerate() {
        let token = String::from_utf8(token.clone()).unwrap();
        println!("Current token {} - '{}'", i, token);
    }
    println!("");

    let eval_result = eval_reversed_polish(&reversed_polish_tokens);
    println!("Result = {}", eval_result);
}