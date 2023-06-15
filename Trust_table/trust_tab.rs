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

fn print_truth_table(formula: &str) {
    let mut operators = VecDeque::new();
    let mut operands = Vec::new();

    for c in formula.chars() {
        if c.is_uppercase() == true {
            operands.push(c);
            print!("{} |", c);
        } else {
            operators.push_back(c);
        }
    }
    print!("= |");
    println!("");
    let n = operands.len();
    let max_iteration = (1 << n) - 1;
    if n == formula.len() {
        println!("you re formula is not valid you must have operators with operands");
        return;
    }
    for i in 0..=max_iteration {
        let mut stack = Vec::new();
        for c in formula.chars() {
            if c.is_uppercase() {
                match find_char(&operands, c) {
                    Some(pos) => {
                        stack.push((i >> pos) & 1 == 1);
                        print!("{} |", if (i >> pos) & 1 == 1 { 1 } else { 0 });
                    }
                    None => {
                        stack.push(false);
                    }
                }
            } else if c == '!' {
                let op1 = stack.pop().unwrap();
                stack.push(!op1);
            } else if c == '&' {
                if stack.len() < 2 {
                    println!("you re formula is not valid you have to got le nbr of 1or 1 - 1 of char special");
                    return;
                }
                let data = stack.pop().unwrap();
                let data1 = stack.pop().unwrap();
                stack.push(data && data1);
            } else if c == '|' {
                if stack.len() < 2 {
                    println!("you re formula is not valid you have to got le nbr of 1or 1 - 1 of char special");
                    return;
                }
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(op1 || op2);
            } else if c == '=' {
                if stack.len() < 2 {
                    println!("you re formula is not valid you have to got le nbr of 1or 1 - 1 of char special");
                    return;
                }
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(op1 == op2);
            } else if c == '>' {
                if stack.len() < 2 {
                    println!("you re formula is not valid you have to got le nbr of 1or 1 - 1 of char special");
                    return;
                }
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(!op1 || op2);
            } else if c == '^' {
                if stack.len() < 2 {
                    println!("you re formula is not valid you have to got le nbr of 1or 1 - 1 of char special");
                    return;
                }
                let op2 = stack.pop().unwrap();
                let op1 = stack.pop().unwrap();
                stack.push(op1 ^ op2);
            }
        }
        if stack.len() > 1 {
            println!("you re formula is not valid");
            return;
        }
        println!("{} |", if stack.pop().unwrap() { 1 } else { 0 });
    }
}

fn main() {
    println!("--------------------------------------");
    println! {" AB&C|"};
    print_truth_table("AB&C|");
    println!("--------------------------------------");
    println! {" AB&"};
    print_truth_table("AB&");
    println!("--------------------------------------");
    println! {"AB^"};
    print_truth_table("AB^");
    println!("--------------------------------------");
    println! {"AA^"};
    print_truth_table("AA^");
    println!("--------------------------------------");
    println! {"A!B!&"};
    print_truth_table("A!C!&");
    println!("--------------------------------------");
    println! {"A!B&"};
    print_truth_table("A!B&");
    println!("--------------------------------------");
    println! {"A!B!&C&"};
    print_truth_table("A!B!&C&");
    println!("--------------------------------------");
    println! {"AB|D|C&"};
    print_truth_table("AB|D|C&");
    println!("--------------------------------------");
    println! {"AB|D|C&|"};
    print_truth_table("AB|D|C&|");
    println!("--------------------------------------");
}
