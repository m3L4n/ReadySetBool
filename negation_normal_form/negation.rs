// http://homepage.divms.uiowa.edu/~hzhang/c188/notes/ch02b-NF.pdf
//  pour le xor (A & !B) | ( !A & B)

use std::collections::VecDeque;
struct Node {
  data: char,
  left: Option<Box<Node>>,
  right: Option<Box<Node>>,
}

impl Node {
  fn new(data :char) -> Node {
      Node {
          data : data,
          left: None,
          right: None,
      }
  }
  fn clone_with_children(&self) -> Box<Node> {
    let left = self.left.as_ref().map(|node| node.clone_with_children());
    let right = self.right.as_ref().map(|node| node.clone_with_children());
    Box::new(Node {
        data: self.data,
        left,
        right,
    })
}
  fn insert_string_into_bst(&mut self,  ch: &mut VecDeque<char>) {
    let value = ch.pop_back();
    self.data = value.unwrap();
    let val: char = value.unwrap();
    if "!&=>|^".chars().any(|c| c == val){
      self.right = Some(Box::new(Node::new('0')));
      self.right.as_mut().unwrap().insert_string_into_bst(ch);
      if "&=>|^".chars().any(|c| c == val){
        self.left = Some(Box::new(Node::new('0')));
        self.left.as_mut().unwrap().insert_string_into_bst(ch);
      }
    }
}
fn get_tree_preorder(&mut self) -> String {
  let mut result = String::from("");
  if let Some(_left) = &self.left {
    result += &self.left.as_mut().unwrap().get_tree_preorder();
  }
  if let Some(_right) = &self.right {
    result += &self.right.as_mut().unwrap().get_tree_preorder();
  }
  result += &format!("{}", self.data);
  result
}
fn move_right_place(&mut self){
  match self.left.as_mut() {
    None => (),
    Some(left) => left.move_right_place(),
}

match self.right.as_mut() {
    None => (),
    Some(right) => right.move_right_place(),
}
    match self.data {
      '!' => {
        if let Some(right) =  &mut self.right {
          if  "!&|".chars().any(|c| c == right.data)
          {
            if right.data == '&' || right.data ==  '|'{
              let mut new_node_right = Node::new('!');
              let mut new_node_left = Node::new('!');
              new_node_right.right = right.right.take();
              new_node_left.right = right.left.take();
              if right.data == '&' {
                self.data = '|';
              }
              else{
                self.data = '&';
              }
              self.left = Some(Box::new(new_node_left));
              self.right = Some(Box::new(new_node_right));
              self.move_right_place();
            }
            else {
              if let Some(right) =  &mut self.right {
                self.data = right.right.as_mut().unwrap().data;
                self.left = self.right.as_mut().unwrap().right.as_mut().unwrap().left.take();
                self.right = self.right.as_mut().unwrap().right.as_mut().unwrap().right.take();
                self.move_right_place();
              }
              
            }
            
          }
        }
      }
      '>' =>{
          self.data = '|';
          let mut new_node_left = Node::new('!');
          new_node_left.right = self.left.take();
          self.left = Some(Box::new(new_node_left));
          self.move_right_place();
      }
      '=' => {
        self.data = '&';
        let mut node_right_left = Node::new(self.right.as_mut().unwrap().data);
        let mut node_right_right = Node::new(self.left.as_mut().unwrap().data);
        let  node_left_left = Node::new(self.left.as_mut().unwrap().data);
        let  node_left_right = Node::new(self.right.as_mut().unwrap().data);
        node_right_right.right = self.right.as_mut().unwrap().right.take();
        node_right_left.left = self.left.as_mut().unwrap().left.take();
        self.right.as_mut().unwrap().data = '>';
        self.left.as_mut().unwrap().data = '>';
        self.right.as_mut().unwrap().right =  Some(Box::new(node_right_right));
        self.right.as_mut().unwrap().left =  Some(Box::new(node_right_left));
        self.left.as_mut().unwrap().right = Some(Box::new(node_left_right));
        self.left.as_mut().unwrap().left =  Some(Box::new(node_left_left));
        self.move_right_place();
      }
      '^' => {
        self.data = '|';
        let mut new_node_neg_right = Node::new('!'); // negatif qui se situe a gauche qui va prendre b
       let mut new_node_neg_left = Node::new('!'); // negatif a droit  qui va prendre a 
       new_node_neg_left.right = Some(self.right.as_mut().unwrap().clone_with_children());// permet qu'il prenne a psk a gauche de self c A
       new_node_neg_right.right = Some(self.left.as_mut().unwrap().clone_with_children()); // permet qu'il prenne a psk a droite de self c ;
       let mut new_node_right = Node::new('&');
       let mut  new_node_left = Node::new('&');
       new_node_left.left = Some(self.left.as_mut().unwrap().clone_with_children());
       new_node_left.right =  Some(Box::new(new_node_neg_left));
       new_node_right.right = Some(self.right.as_mut().unwrap().clone_with_children());
       new_node_right.left =  Some(Box::new(new_node_neg_right));
      self.right =  Some(Box::new(new_node_right));
      self.left =  Some(Box::new(new_node_left));
    }
      _ => {
      }
    }
    }
}

fn parse(root : &mut Node ,string :&str){
  let mut string_chars = string.chars().collect::<VecDeque<_>>();
    root.insert_string_into_bst( &mut string_chars);
}
fn negation_normal_form(formula: &str) -> String
{
  let mut root = Node::new('1');
  parse(&mut root, formula);
  root.move_right_place();
  return root.get_tree_preorder();
}
fn main(){
  println!("{}", negation_normal_form("AB&!"));
  // A!B!|
  println!("{}", negation_normal_form("AB|!"));
  // A!B!&
  println!("{}", negation_normal_form("AB>"));
  // A!B|
  println!("{}", negation_normal_form("AB="));
  // AB&A!B!&|
  println!("{}", negation_normal_form("AB|C&!"));
  // A!B!&C!|
}