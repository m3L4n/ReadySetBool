
fn eval_formula(formula: &str) -> bool{

  let mut stack = Vec::new();
  for c in formula.chars() {
    match c {
      '&' =>{
        if stack.len() < 2{
          println!("you re formula is not valid you have to got le nbr of 1or 1 - 1 of char special");
          return false;
        }
        let op2 = stack.pop().unwrap();
        let op1 = stack.pop().unwrap();
        stack.push(op1 && op2);
    }
    '|' =>{
      if stack.len() < 2{
        println!("you re formula is not valid you have to got le nbr of 1or 1 - 1 of char special");
        return false;
      }
      let op2 = stack.pop().unwrap();
      let op1 = stack.pop().unwrap();
      stack.push(op1 || op2);
  }
    '=' =>{
      if stack.len() < 2{
        println!("you re formula is not valid you have to got le nbr of 1or 1 - 1 of char special");
        return false;
      }
      let op2 = stack.pop().unwrap();
      let op1 = stack.pop().unwrap();
      stack.push(op1 == op2);
  }
  '>' =>{
    if stack.len() < 2{
      println!("you re formula is not valid you have to got le nbr of 1or 1 - 1 of char special");
      return false;
    }
    let op2 = stack.pop().unwrap();
    let op1 = stack.pop().unwrap();
    stack.push(!op1 || op2);
}
  '^' =>{
    if stack.len() < 2{
      println!("you re formula is not valid");
      return false;
    }
    let op2 = stack.pop().unwrap();
    let op1 = stack.pop().unwrap();
    stack.push(op1 ^ op2);
}
  '!' =>{
    if stack.len() < 1{
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
if stack.len() != 1{
  println!("youre formula is not goot");
  return false;
}
return stack.pop().unwrap();
}

fn main(){
  println!(" 10& = {}", eval_formula("10&"));
  println!(" 10| = {}", eval_formula("10|"));
  println!("10> {}", eval_formula("10>"));
  println!("11> {}", eval_formula("11>"));
  println!(" 01> {}", eval_formula("01>"));
  println!(" 00> {}", eval_formula("00>"));
// true
println!("10= ={}", eval_formula("10="));
// false
println!("1011||= {}", eval_formula("1011||="));
// true
println!("1011|| {}", eval_formula("1011||"));
println!("101||!{}", eval_formula("101||!"));
}