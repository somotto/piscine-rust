use std::env;

fn rpn(expr: &str) {
    let mut stack: Vec<i64> = Vec::new();

    for token in expr.split_whitespace() {
        match token {
            "+" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a + b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "-" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a - b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "*" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a * b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "/" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    if b == 0 {
                        println!("Error");
                        return;
                    }
                    stack.push(a / b);
                } else {
                    println!("Error");
                    return;
                }
            }
            "%" => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    if b == 0 {
                        println!("Error");
                        return;
                    }
                    stack.push(a % b);
                } else {
                    println!("Error");
                    return;
                }
            }
            num => match num.parse::<i64>() {
                Ok(n) => stack.push(n),
                Err(_) => {
                    println!("Error");
                    return;
                }
            },
        }
    }

    if stack.len() == 1 {
        println!("{}", stack[0]);
    } else {
        println!("Error");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error");
        return;
    }

    rpn(&args[1]);
}
