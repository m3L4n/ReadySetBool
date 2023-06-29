use crate::truth_operations::boolean_evaluation::eval_formula;
use std::collections::VecDeque;

fn find_char(vec: &Vec<char>, c: char) -> Option<usize> {
    for (i, &item) in vec.iter().enumerate() {
        if item == c {
            return Some(i);
        }
    }
    None
}

fn truth_table(n: usize) -> Vec<Vec<u32>> {
    let rows = 1 << n;
    let mut table: Vec<Vec<u32>> = Vec::with_capacity(rows as usize);

    for i in 0..rows {
        let mut row = Vec::with_capacity(n);
        for j in 0..n {
            let val = (i >> j) & 1;
            row.push(val);
        }
        table.push(row);
    }

    table
}

fn transform_and_eval(row: &mut Vec<u32>, formula: &str, operands: &Vec<char>) {
    let mut tmp_formula = String::from(formula);
    for (index, c) in formula.chars().enumerate() {
        if c.is_uppercase() {
            match find_char(operands, c) {
                Some(pos) => {
                    let replacement: char = if row[pos] != 0 { '1' } else { '0' };
                    tmp_formula.replace_range(index..=index, &replacement.to_string());
                }
                None => {}
            }
        }
    }
    row.push(eval_formula(&tmp_formula) as u32);
}
pub fn print_truth_table(formula: &str) {
    let mut operators = VecDeque::new();
    let mut operands = Vec::new();

    for c in formula.chars() {
        if c.is_uppercase() && !operands.contains(&c) {
            operands.push(c);
            print!("{} |", c);
        } else {
            operators.push_back(c);
        }
    }
    println!("= |");
    let mut matrix_truth_table = truth_table(operands.len());
    for (_index, row) in matrix_truth_table.iter_mut().enumerate() {
        let mut row_clone = row.clone();
        transform_and_eval(&mut row_clone, formula, &operands);
        *row = row_clone;
        for c in row.clone() {
            print!("{} |", c);
        }
        println!();
    }
}
pub fn test_truthtable() {
    println!("--------------------------------------");
    println! {" AB&C|"};
    print_truth_table("AB&C|");
    println!("--------------------------------------");
    println! {" AB&"};
    print_truth_table("AB&");
    println!("--------------------------------------");
    println! {" AB|"};
    print_truth_table("AB|");
    println!("--------------------------------------");
    println! {" A!B|"};
    print_truth_table("A!B|");
    println!("--------------------------------------");
    println! {" AB|!"};
    print_truth_table("AB|!");
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
    println! {"AB&B&CA!||"};
    print_truth_table("AB&B&C|A!|");
    println!("--------------------------------------");
    println! {"A"};
    print_truth_table("A");
    println!("--------------------------------------");
    println! {"B!"};
    print_truth_table("B!");
    println!("--------------------------------------");
    println! {"ABC^^"};
    print_truth_table("ABC^^");
    println!("--------------------------------------");
    println! {"AB>A>A>"};
    print_truth_table("AB>A>A>");
    println!("--------------------------------------");
}
