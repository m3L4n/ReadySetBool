use crate::truth_operations::boolean_evaluation::eval_formula;
use crate::truth_operations::truth_table::print_truth_table;
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

fn transform_and_eval(row: &mut Vec<u32>, formula: &str, operands: &Vec<char>) -> bool {
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
    let result = eval_formula(&tmp_formula);
    if result {
        return true;
    }
    row.push(result as u32);
    false
}
fn print_truth_table_boolean(formula: &str) -> bool {
    let mut operators = VecDeque::new();
    let mut operands = Vec::new();

    for c in formula.chars() {
        if c.is_uppercase() && !operands.contains(&c) {
            operands.push(c);
            // print!("{} |", c);
        } else {
            operators.push_back(c);
        }
    }
    // println!("= |");
    let mut matrix_truth_table = truth_table(operands.len());
    for (_index, row) in matrix_truth_table.iter_mut().enumerate() {
        let mut row_clone = row.clone();
        let res = transform_and_eval(&mut row_clone, formula, &operands);
        if res {
            return true;
        }
        *row = row_clone;
        // for c in row.clone() {
        // print!("{} |", c);
        // }
        // println!("");
    }
    false
}

fn sat(formula: &str) -> bool {
    print_truth_table_boolean(formula)
}
pub fn test_sat() {
    println!("----------------------------");
    println!(" TEST WITH A");
    println!(" truth table : ");
    print_truth_table("A");
    println!("A=   {}", sat("A"));
    assert_eq!(sat("A"), true);
    println!("----------------------------");
    println!(" TEST WITH A!");
    println!(" truth table : ");
    print_truth_table("A!");
    println!("A!=   {}", sat("A!"));
    assert_eq!(sat("A!"), true);
    println!("----------------------------");
    println!(" TEST WITH A!B>");
    println!(" truth table : ");
    print_truth_table("A!B>");
    println!("A!B>=   {}", sat("A!B>"));
    assert_eq!(sat("A!B>"), true);
    println!("----------------------------");
    println!(" TEST WITH AB|");
    println!(" truth table : ");
    print_truth_table("AB|");
    println!("AB|  = {}", sat("AB|"));

    assert_eq!(sat("AB|"), true);
    println!("----------------------------");
    println!(" TEST WITH AB&");
    println!(" truth table : ");
    print_truth_table("AB&");
    println!("AB&  = {}", sat("AB&"));
    assert_eq!(sat("AB&"), true);
    println!("----------------------------");
    println!(" TEST WITH AA!&");
    println!(" truth table : ");
    print_truth_table("AA!&");
    println!("AA!&  = {}", sat("AA!&"));
    assert_eq!(sat("AA!&"), false);
    println!("----------------------------");
    println!(" TEST WITH AA^");
    println!(" truth table : ");
    print_truth_table("AA^");
    println!("AA^ =   {}", sat("AA^"));
    assert_eq!(sat("AA^"), false);
    println!("----------------------------");
    println!(" TEST WITH A!B&");
    println!(" truth table : ");
    print_truth_table("A!B&");
    println!("A!B& =   {}", sat("A!B&"));
    assert_eq!(sat("A!B&"), true);
    println!("----------------------------");
    println!(" TEST WITH AB|D|C&");
    println!(" truth table : ");
    print_truth_table("AB|D|C&");
    println!("AB|D|C& =   {}", sat("AB|D|C&"));
    assert_eq!(sat("AB|D|C&"), true);
    println!("----------------------------");
    println!(" TEST WITH ABC||");
    println!(" truth table : ");
    print_truth_table("ABC||");
    println!("ABC|| =   {}", sat("ABC||"));
    assert_eq!(sat("ABC||"), true);
    println!("----------------------------");
    println!(" TEST WITH AB&A!B!&&");
    println!(" truth table : ");
    print_truth_table("AB&A!B!&&");
    println!("AB&A!B!&& =   {}", sat("AB&A!B!&&"));
    assert_eq!(sat("AB&A!B!&&"), false);
    println!("----------------------------");
    println!(" TEST WITH ABCDE&&&&");
    println!(" truth table : ");
    print_truth_table("ABCDE&&&&");
    println!("ABCDE&&&&=   {}", sat("ABCDE&&&&"));
    assert_eq!(sat("ABCDE&&&&"), true);
    println!("----------------------------");
    println!(" TEST WITH AAA^^");
    println!(" truth table : ");
    print_truth_table("AAA^^");
    println!("AAA^^=   {}", sat("AAA^^"));
    assert_eq!(sat("AAA^^"), true);
    println!("----------------------------");
    println!(" TEST WITH ABCDE^^^^");
    println!(" truth table : ");
    print_truth_table("ABCDE^^^^");
    println!("ABCDE^^^^=   {}", sat("ABCDE^^^^"));
    assert_eq!(sat("ABCDE^^^^"), true);
}
