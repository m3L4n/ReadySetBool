use std::collections::VecDeque;
/**  La complexité O(2^n) signifie que le temps d'exécution de l'algorithme augmente de manière exponentielle en fonction de la taille de l'entrée n.
Cela signifie que si n augmente de 1, le nombre d'opérations nécessaires pour exécuter l'algorithme double.
**/
fn find_char(vec: &Vec<char>, c: char) -> Option<usize> {
    for (i, &item) in vec.iter().enumerate() {
        if item == c {
            return Some(i);
        }
    }
    None
}
fn print_truth_table(formula: &str) -> bool {
    let mut operators = VecDeque::new();
    let mut operands = Vec::new();
    let mut is_true = false;

    for c in formula.chars() {
        if c.is_uppercase() == true {
            operands.push(c);
        } else {
            operators.push_back(c);
        }
    }
    let n = operands.len();
    let max_iteration = (1 << n) - 1;
    for i in 0..=max_iteration {
        let mut stack = Vec::new();
        for c in formula.chars() {
            if c.is_uppercase() {
                match find_char(&operands, c) {
                    Some(pos) => {
                        stack.push((i >> pos) & 1 == 1);
                    }
                    None => {}
                }
            } else if c == '!' {
                if stack.len() < 1 {
                    println!("problem with the formula");
                    return false;
                }
                let op1 = stack.pop().unwrap();
                stack.push(!op1);
            } else if c == '&' {
                if stack.len() < 2 {
                    println!("problem with the formula");
                    return false;
                }
                let data = stack.pop().unwrap();
                let data1 = stack.pop().unwrap();
                stack.push(data && data1);
            } else if c == '|' {
                if stack.len() < 2 {
                    println!("problem with the formula");
                    return false;
                }
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(op1 || op2);
            } else if c == '=' {
                if stack.len() < 2 {
                    println!("problem with the formula");
                    return false;
                }
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(op1 == op2);
            } else if c == '>' {
                if stack.len() < 2 {
                    println!("problem with the formula");
                    return false;
                }
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(!op1 || op2);
            } else if c == '^' {
                if stack.len() < 2 {
                    println!("problem with the formula");
                    return false;
                }
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(op1 ^ op2);
            }
        }
        if stack.len() != 1 {
            println!("problem with the formula");
            return false;
        }
        if stack.pop().unwrap() {
            is_true = true
        };
    }
    return is_true;
}

fn sat(formula: &str) -> bool {
    return print_truth_table(formula);
}

fn main() {
    println!("AB|  = {}", sat("AB|"));
    // true
    println!("AB&  = {}", sat("AB&"));
    // true
    println!("AA!&  = {}", sat("AA!&"));
    // false
    println!("AA^ =   {}", sat("AA^"));
    // false
    println!("A!B& =   {}", sat("A!B&"));
    println!("AB|D|C& =   {}", sat("AB|D|C&"));
    println!("AB|D|C&| =   {}", sat("AB|D|C&|"));
}
