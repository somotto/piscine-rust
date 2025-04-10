fn rpn(expr: &str) -> Result<i64, &'static str> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    let mut stack: Vec<i64> = Vec::new();

    for token in tokens {
        match token {
            "+" | "-" | "*" | "/" | "%" => {
                if stack.len() < 2 {
                    return Err("Error");
                }
                
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0 {
                            return Err("Error");
                        }
                        a / b
                    },
                    "%" => {
                        if b == 0 {
                            return Err("Error");
                        }
                        a % b
                    },
                    _ => unreachable!(),
                };
                
                stack.push(result);
            },
            _ => {
                match token.parse::<i64>() {
                    Ok(num) => stack.push(num),
                    Err(_) => return Err("Error"),
                }
            }
        }
    }
    
    if stack.len() != 1 {
        return Err("Error");
    }
    
    Ok(stack[0])
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 2 {
        println!("Error");
        return;
    }
    
    match rpn(&args[1]) {
        Ok(result) => println!("{}", result),
        Err(err) => println!("{}", err),
    }
}