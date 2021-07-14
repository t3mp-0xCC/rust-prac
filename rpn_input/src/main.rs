use std::io;

fn main() {
    println!("Reverse Polish Notation\nRust Edition");
    println!("Please input your RPN");
    
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)
        .expect("Input Err!");
    let ans = rpn(&input);

    println!("{} = {:.4}", input, ans);
}

fn rpn(input: &str) -> f64 {
    // vector seems like stack
    let mut stack = Vec::new();
    // split stings by space
    for token in input.split_whitespace() {
        // if token == f64, push stack
        if let Ok(num) = token.parse::<f64>(){
            stack.push(num);
        } else {
            // if token = operator
            match token {
                // |x, y| x + y -> closure
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),

                _ => panic!("Unknown operator:{}", token),
                }
        }
    }
    stack.pop().expect("Stack Underflow!")
}

fn apply2<F>(stack: &mut Vec<f64>, fun: F)
where
    F: Fn(f64, f64) -> f64,
{
    if let(Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        let z = fun(x, y);
        stack.push(z);
    } else {
        // if stack.pop() = NULL
        panic!("Stack Underflow!");
    }
}

