fn find_char(vec: &Vec<char>, c: char) -> usize{
  for (i, &item) in vec.iter().enumerate() {
      if item == c {
          return i;
      }
  }
  return 0;

}

fn eval_set(formula: &str,sets: &[&[i32]]) -> Vec<i32>{
  let mut variables  = Vec::new();
  let mut operations = Vec::new();
  let mut result = vec![];
  for c in formula.chars() {
    if c.is_ascii_uppercase()
    {
      variables.push(c);
      operations.push(c);
    }
    if  c == '&'{
      let  op1 = find_char(&variables, operations.pop().unwrap());
      let  op2 = find_char(&variables, operations.pop().unwrap());
      let max_iteration = sets[0].len() - 1;
      for i in 0..=max_iteration {
        let op1_bits = 1 << sets[op1][i]; 
        let op2_bits = 1 << sets[op2][i];

      if (op1_bits & op2_bits) != 0 {
        //  if sets[op1][i] && sets[op2]{
            result.push(sets[op1][i]);
            result.push(sets[op2][i]);
          }
        }
  }
  if  c == '|'{
    let  op1 = find_char(&variables, operations.pop().unwrap());
    let  op2 = find_char(&variables, operations.pop().unwrap());
    let max_iteration = sets[0].len() - 1;
    for i in 0..=max_iteration {
      let op1_bits = 1 << sets[op1][i];
      let op2_bits = 1 << sets[op2][i];

    if (op1_bits | op2_bits) != 0 {
      //  if sets[op1][i] && sets[op2]{
          result.push(sets[op1][i]);
          result.push(sets[op2][i]);
        }
      }
}
if  c == '>'{
  let  op1 = find_char(&variables, operations.pop().unwrap());
  let  op2 = find_char(&variables, operations.pop().unwrap());
  let max_iteration = sets[0].len() - 1;
  for i in 0..=max_iteration {
    let op1_bits = 1 << sets[op1][i];
    let op2_bits = 1 << sets[op2][i];

  if op1_bits > op2_bits {
    //  if sets[op1][i] && sets[op2]{
        result.push(sets[op1][i]);
        result.push(sets[op2][i]);
      }
    }
}
if  c == '!'{
  let  op1 = find_char(&variables, operations.pop().unwrap());
  let max_iteration = sets[0].len() - 1;
  for i in 0..=max_iteration {
    let op1_bits = 1 << sets[op1][i];

  if !op1_bits == 1{
    //  if sets[op1][i] && sets[op2]{
        result.push(sets[op1][i]);
      }
    }
}

if  c == '^'{
  let  op1 = find_char(&variables, operations.pop().unwrap());
  let  op2 = find_char(&variables, operations.pop().unwrap());
  let max_iteration = sets[0].len() - 1;
  for i in 0..=max_iteration {
    let op1_bits = 1 << sets[op1][i];
    let op2_bits = 1 << sets[op2][i];

  if op1_bits ^ op2_bits != 0 {
    //  if sets[op1][i] && sets[op2]{
        result.push(sets[op1][i]);
        result.push(sets[op2][i]);
      }
    }
}
if  c == '='{
  let  op1 = find_char(&variables, operations.pop().unwrap());
  let  op2 = find_char(&variables, operations.pop().unwrap());
  let max_iteration = sets[0].len() - 1;
  for i in 0..=max_iteration {
    let op1_bits = 1 << sets[op1][i];
    let op2_bits = 1 << sets[op2][i];

  if op1_bits == op2_bits {
    //  if sets[op1][i] && sets[op2]{
        result.push(sets[op1][i]);
        result.push(sets[op2][i]);
      }
    }
}
}
  return result;
}

fn main(){

  let matrix = [
    [0, 1, 2],
    [3, 4, 5],
];
let matrix1 = [
  [0, 1, 2],
  [0, 3, 4],
];
let matrix2 = [
  [0, 1, 2],
];

let matrix_ref: &[&[i32]] = &matrix.iter().map(|row| row.as_ref()).collect::<Vec<_>>();
let matrix_ref1: &[&[i32]] = &matrix1.iter().map(|row| row.as_ref()).collect::<Vec<_>>();
let matrix_ref2: &[&[i32]] = &matrix2.iter().map(|row| row.as_ref()).collect::<Vec<_>>();
println!("{:?}", eval_set("AB&",&matrix_ref1));
 println!("{:?}", eval_set("AB|",&matrix_ref));
 println!("{:?}", eval_set("A!",&matrix_ref2));
}
