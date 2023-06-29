// http://homepage.divms.uiowa.edu/~hzhang/c188/notes/ch02b-NF.pdf
//  pour le xor (A & !B) | ( !A & B)

use crate::truth_operations::truth_table::print_truth_table;
use std::collections::VecDeque;
struct Node {
    data: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(data: char) -> Node {
        Node {
            data,
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
    fn insert_string_into_bst(&mut self, ch: &mut VecDeque<char>) {
        let value = ch.pop_back();
        self.data = value.unwrap();
        let val: char = value.unwrap();
        if "!&=>|^".chars().any(|c| c == val) {
            self.right = Some(Box::new(Node::new('0')));
            self.right.as_mut().unwrap().insert_string_into_bst(ch);
            if "&=>|^".chars().any(|c| c == val) {
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
    fn move_right_place(&mut self) {
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
                if let Some(right) = &mut self.right {
                    if "!&|".chars().any(|c| c == right.data) {
                        if right.data == '&' || right.data == '|' {
                            let mut new_node_right = Node::new('!');
                            let mut new_node_left = Node::new('!');
                            new_node_right.right = right.right.take();
                            new_node_left.right = right.left.take();
                            if right.data == '&' {
                                self.data = '|';
                            } else {
                                self.data = '&';
                            }
                            self.left = Some(Box::new(new_node_left));
                            self.right = Some(Box::new(new_node_right));
                            self.move_right_place();
                        } else if let Some(right) = &mut self.right {
                            self.data = right.right.as_mut().unwrap().data;
                            self.left = self
                                .right
                                .as_mut()
                                .unwrap()
                                .right
                                .as_mut()
                                .unwrap()
                                .left
                                .take();
                            self.right = self
                                .right
                                .as_mut()
                                .unwrap()
                                .right
                                .as_mut()
                                .unwrap()
                                .right
                                .take();
                            self.move_right_place();
                        }
                    }
                }
            }
            '>' => {
                self.data = '|';
                let mut new_node_left = Node::new('!');
                new_node_left.right = self.left.take();
                self.left = Some(Box::new(new_node_left));
                self.move_right_place();
            }
            '=' => {
                self.data = '&';
                let mut node_right = Node::new('>');
                let mut node_left = Node::new('>');
                node_left.left = Some(self.left.as_mut().unwrap().clone_with_children());
                node_left.right = Some(self.right.as_mut().unwrap().clone_with_children());
                node_right.left = self.right.take();
                node_right.right = self.left.take();
                self.right = Some(Box::new(node_right));
                self.left = Some(Box::new(node_left));
                self.move_right_place();
            }
            '^' => {
                self.data = '|';
                let mut new_node_neg_right = Node::new('!');
                let mut new_node_neg_left = Node::new('!');
                new_node_neg_left.right = Some(self.right.as_mut().unwrap().clone_with_children()); // permet qu'il prenne a psk a gauche de self c A
                new_node_neg_right.right = Some(self.left.as_mut().unwrap().clone_with_children()); // permet qu'il prenne a psk a droite de self c ;
                let mut new_node_right = Node::new('&');
                let mut new_node_left = Node::new('&');
                new_node_left.left = Some(self.left.as_mut().unwrap().clone_with_children());
                new_node_left.right = Some(Box::new(new_node_neg_left));
                new_node_right.right = Some(self.right.as_mut().unwrap().clone_with_children());
                new_node_right.left = Some(Box::new(new_node_neg_right));
                self.right = Some(Box::new(new_node_right));
                self.left = Some(Box::new(new_node_left));
                self.move_right_place();
            }
            _ => {}
        }
    }
}

fn parse(root: &mut Node, string: &str) {
    let mut string_chars = string.chars().collect::<VecDeque<_>>();
    root.insert_string_into_bst(&mut string_chars);
}
pub fn negation_normal_form(formula: &str) -> String {
    let mut root = Node::new('1');
    parse(&mut root, formula);
    root.move_right_place();
    root.get_tree_preorder()
}
pub fn test_nnf() {
    println!("AB|C&! =\n-> {}", negation_normal_form("AB|C&!"));
    println!("--------------------------------------");
    println!(" TEST WITH A!B!&!\n");
    println!("A!B!&! =\n-> {}\n", negation_normal_form("A!B!&!"));
    println!(" truth table A!B!&! ");
    print_truth_table("A!B!&!");
    println!(" truth table result nnf A!B!&!");
    print_truth_table(&negation_normal_form("A!B!&!"));
    println!("--------------------------------------");
    println!(" TEST WITH AB^\n");
    println!("AB^ =\n-> {}\n", negation_normal_form("AB^"));
    println!(" truth table AB^ ");
    print_truth_table("AB^");
    println!(" truth table result nnf AB^");
    print_truth_table(&negation_normal_form("AB^"));
    println!("--------------------------------------");
    println!(" TEST WITH AB&!\n");
    println!("AB&! =\n-> {}\n", negation_normal_form("AB&!"));
    println!(" truth table AB&! ");
    print_truth_table("AB&!");
    println!(" truth table result nnf AB&!");
    print_truth_table(&negation_normal_form("AB&!"));
    println!("--------------------------------------");
    println!(" TEST WITH AB>!\n");
    println!("AB>! =\n-> {}\n", negation_normal_form("AB>!"));
    println!(" truth table AB>! ");
    print_truth_table("AB>!");
    println!(" truth table result nnf AB>!");
    print_truth_table(&negation_normal_form("AB>!"));
    println!("--------------------------------------");
    println!(" TEST WITH AB=\n");
    println!("AB= =\n-> {}\n", negation_normal_form("AB="));
    println!(" truth table AB= ");
    print_truth_table("AB=");
    println!(" truth table result nnf AB=");
    print_truth_table(&negation_normal_form("AB="));
    println!("--------------------------------------");
    println!(" TEST WITH ABC^=\n");
    println!("ABC= =\n-> {}\n", negation_normal_form("ABC^="));
    println!(" truth table ABC^= ");
    print_truth_table("ABC^=");
    println!(" truth table result nnf ACB^=");
    print_truth_table(&negation_normal_form("ABC^="));
    println!("--------------------------------------");
    println!(" TEST WITH ABC=^\n");
    println!("ABC=^ =\n-> {}\n", negation_normal_form("ABC=^"));
    println!(" truth table ABC=^ ");
    print_truth_table("ABC=^");
    println!(" truth table result nnf ACB=^");
    print_truth_table(&negation_normal_form("ABC=^"));
    println!("--------------------------------------");
    println!(" TEST WITH AB=!\n");
    println!("AB=! =\n-> {}\n", negation_normal_form("AB=!"));
    println!(" truth table AB=! ");
    print_truth_table("AB=!");
    println!(" truth table result nnf AB=!");
    print_truth_table(&negation_normal_form("AB=!"));
    println!("--------------------------------------");
    println!(" TEST WITH ABC&|\n");
    println!("ABC&| =\n-> {}\n", negation_normal_form("ABC&|"));
    println!(" truth table ABC&|");
    print_truth_table("ABC&|");
    println!(" truth table result nnf ABC&|");
    print_truth_table(&negation_normal_form("ABC&|"));
    println!("--------------------------------------");
    println!(" TEST WITH ABC||!\n");
    println!("ABC||! =\n-> {}\n", negation_normal_form("ABC||!"));
    println!(" truth table ABC||!");
    print_truth_table("ABC||!");
    println!(" truth table result nnf ABC||!");
    print_truth_table(&negation_normal_form("ABC||!"));
    println!("--------------------------------------");
    println!(" TEST WITH ABC&|!\n");
    println!(" ABC&|! =\n-> {}\n", negation_normal_form("ABC&|!"));
    println!(" truth table ABC&|!");
    print_truth_table("ABC&|!");
    println!(" truth table result nnf ABC&|!");
    print_truth_table(&negation_normal_form("ABC&|!"));
    println!("--------------------------------------");
    println!(" TEST WITH ABC^^\n");
    println!("ABC^^=\n-> {}\n", negation_normal_form("ABC^^"));
    println!(" \ntruth table ABC^^");
    print_truth_table("ABC^^");
    println!(" truth table result nnfABC^^^ ");
    print_truth_table(&negation_normal_form("ABC^^"));
    println!("--------------------------------------");
    println!(" TEST WITH ABC>>\n");
    println!("ABC>> =\n-> {}\n", negation_normal_form("ABC>>"));
    println!(" truth table ABC^^^");
    print_truth_table("ABC>>");
    println!(" truth table result nnf ABC>>");
    print_truth_table(&negation_normal_form("ABC>>"));
    println!("--------------------------------------");
    println!(" TEST WITH AB|!C^A|C=\n");
    println!("ABC>> =\n-> {}\n", negation_normal_form("ABCD|!^&"));
    println!(" truth table AB|!C^A|C=");
    print_truth_table("ABCD|!^&");
    println!(" truth table result nnf ABCD|!^&");
    print_truth_table(&negation_normal_form("ABCD|!^&"));
}
