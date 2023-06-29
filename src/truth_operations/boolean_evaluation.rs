pub fn eval_formula(formula: &str) -> bool {
    let mut stack = Vec::new();
    for c in formula.chars() {
        match c {
            '&' => {
                if stack.len() < 2 {
                    println!("you re formula is not valid you have to got le nbr of 1or 1 - 1 of char special");
                    return false;
                }
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(op1 && op2);
            }
            '|' => {
                if stack.len() < 2 {
                    println!("you re formula is not valid you have to got le nbr of 1or 1 - 1 of char special");
                    return false;
                }
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(op1 || op2);
            }
            '=' => {
                if stack.len() < 2 {
                    println!("you re formula is not valid you have to got le nbr of 1or 1 - 1 of char special");
                    return false;
                }
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(op1 == op2);
            }
            '>' => {
                if stack.len() < 2 {
                    println!("you re formula is not valid you have to got le nbr of 1or 1 - 1 of char special");
                    return false;
                }
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(!op1 || op2);
            }
            '^' => {
                if stack.len() < 2 {
                    println!("you re formula is not valid");
                    return false;
                }
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(op1 ^ op2);
            }
            '!' => {
                if stack.is_empty() {
                    println!("you re formula is not valid");
                    return false;
                }
                let op1 = stack.pop().unwrap();
                stack.push(!op1);
            }
            '0' => {
                stack.push(false);
            }
            '1' => {
                stack.push(true);
            }
            _ => {}
        }
    }
    if stack.len() != 1 {
        println!("youre formula is not goot");
        return false;
    }
    stack.pop().unwrap()
}

pub fn test_boolean() {
    println!("-------------------------------");
    println!("10& = {}", eval_formula("10&"));
    assert_eq!(eval_formula("10&"), false);
    println!("-------------------------------");
    println!("01& = {}", eval_formula("01&"));
    assert_eq!(eval_formula("01&"), false);
    println!("-------------------------------");
    println!("10| = {}", eval_formula("10|"));
    assert_eq!(eval_formula("01|"), true);
    println!("-------------------------------");
    println!("10> {}", eval_formula("10>"));
    assert_eq!(eval_formula("10>"), false);
    println!("-------------------------------");
    println!("11> {}", eval_formula("11>"));
    assert_eq!(eval_formula("11>"), true);
    println!("-------------------------------");
    println!("01> {}", eval_formula("01>"));
    assert_eq!(eval_formula("01>"), true);
    println!("-------------------------------");
    println!("00> {}", eval_formula("00>"));
    assert_eq!(eval_formula("00>"), true);
    println!("-------------------------------");
    println!("10= {}", eval_formula("10="));
    assert_eq!(eval_formula("10="), false);
    println!("-------------------------------");
    println!("0 {}", eval_formula("0"));
    assert_eq!(eval_formula("0"), false);
    println!("-------------------------------");
    println!("00>0> {}", eval_formula("00>0>"));
    assert_eq!(eval_formula("00>0>"), false);
    println!("-------------------------------");
    println!("1! {}", eval_formula("1!"));
    assert_eq!(eval_formula("1!"), false);
    println!("-------------------------------");
    println!("1 {}", eval_formula("1"));
    assert_eq!(eval_formula("1"), true);
    println!("-------------------------------");
    println!("0 {}", eval_formula("0!"));
    assert_eq!(eval_formula("0!"), true);
    println!("-------------------------------");
    println!("1011||= {}", eval_formula("1011||="));

    assert_eq!(eval_formula("1011||="), true);
    println!("-------------------------------");
    println!("01&0|1=! {}", eval_formula("01&0|1=!"));
    assert_eq!(eval_formula("01&0|1=!"), true);

    println!("-------------------------------");
    println!("01&0|1= {}", eval_formula("01&0|1="));
    assert_eq!(eval_formula("01&0|1="), false);

    println!("-------------------------------");
}
