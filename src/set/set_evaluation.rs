use std::collections::HashSet;

// https://lecluseo.scenari-community.org/2DE/Probabilites/co/G_IntersectionUnion.html
// https://lecluseo.scenari-community.org/2DE/Probabilites/co/G_probacontraire.html
/*
A&B|C"
A = {1, 2, 3}
B = {2, 3, 4}
A&B = {2, 3}
C = {3, 4, 5}
 == {2, 3, 4, 5}*/

use std::collections::HashMap;

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
fn negation_normal_form(formula: &str) -> String {
    let mut root = Node::new('1');
    parse(&mut root, formula);
    root.move_right_place();
    root.get_tree_preorder()
}

fn map_chars_to_indexes(var: &Vec<char>, sets: &Vec<Vec<i32>>) -> HashMap<char, Vec<i32>> {
    let mut map: HashMap<char, Vec<i32>> = HashMap::new();

    for (i, set) in sets.iter().enumerate() {
        if set.is_empty() {
            let key = var[i];
            map.entry(key).or_default();
        }
        for &num in set {
            // println!(" num :{} {:?} i {}", num, set, i);
            let key = var[i];
            let indexes = map.entry(key).or_default();
            indexes.push(num);
        }
    }
    map
}

fn get_unique_uppercase_chars(s: &str) -> Vec<char> {
    let mut result = Vec::new();
    let mut unique_chars = HashSet::new();
    for c in s.chars() {
        if c.is_ascii_uppercase() && !unique_chars.contains(&c.to_ascii_uppercase()) {
            result.push(c.to_ascii_uppercase());
            unique_chars.insert(c.to_ascii_uppercase());
        }
    }
    result
}

fn set_union(set1: &Vec<i32>, set2: &Vec<i32>) -> Vec<i32> {
    // V  |
    let mut result: Vec<i32> = set1.clone();
    for elem in set2 {
        if !result.contains(elem) {
            result.push(*elem);
        }
    }
    result
}
fn set_intersection(set1: &Vec<i32>, set2: &Vec<i32>) -> Vec<i32> {
    //  &
    let result: Vec<i32> = set1.clone();
    let mut res: Vec<i32> = vec![];
    for elem in set2 {
        if result.contains(elem) {
            res.push(*elem);
        }
    }
    res
}

fn set_inverse(set1: &Vec<i32>, set2: &Vec<i32>) -> Vec<i32> {
    let result: Vec<i32> = set1.clone();
    let res_set2: Vec<i32> = set2.clone();
    let mut res: Vec<i32> = vec![];
    for elem in result {
        if !res_set2.contains(&elem) {
            res.push(elem);
        }
    }
    res
}

fn get_the_super_set(sets: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut super_set: Vec<i32> = vec![];
    for (_i, set) in sets.iter().enumerate() {
        for &num in set {
            if !super_set.contains(&num) {
                super_set.push(num)
            }
        }
    }
    super_set
}
fn number_set_formula_match(formula: &Vec<char>, sets: &Vec<Vec<i32>>) -> bool {
    if formula.len() != sets.len() {
        return false;
    }
    true
}
fn return_set_valid(sets: &Vec<Vec<i32>>) -> Vec<i32> {
    for (_i, set) in sets.iter().enumerate() {
        if !set.is_empty() {
            return set.clone();
        }
    }
    return vec![];
}
pub fn eval_set(formula: &str, sets: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut stack: Vec<char> = vec![];
    let mut result: Vec<i32> = vec![];

    let var_within_doble = get_unique_uppercase_chars(formula);
    if !number_set_formula_match(&var_within_doble, sets) {
        return return_set_valid(sets);
    }
    let variables = map_chars_to_indexes(&var_within_doble, sets);
    let super_set = get_the_super_set(sets);
    let new_formula_nnf = negation_normal_form(formula);
    for c in new_formula_nnf.chars() {
        if c == '|' {
            let first_set = stack.pop().unwrap();
            let second_set = stack.pop().unwrap();
            if first_set == 'r' && second_set == 'r' {
                result = set_union(&result, &result);
            } else if first_set != 'r' && second_set == 'r' {
                result = set_union(&variables[&first_set], &result);
            } else if first_set == 'r' && second_set != 'r' {
                result = set_union(&variables[&second_set], &result);
            } else if first_set != 'r' && second_set != 'r' {
                result = set_union(&variables[&first_set], &variables[&second_set]);
            }
            stack.push('r');
        } else if c == '&' {
            let first_set = stack.pop().unwrap();
            let second_set = stack.pop().unwrap();
            if first_set == 'r' && second_set == 'r' {
                result = set_intersection(&result, &result);
            } else if first_set != 'r' && second_set == 'r' {
                result = set_intersection(&variables[&first_set], &result);
            } else if first_set == 'r' && second_set != 'r' {
                result = set_intersection(&variables[&second_set], &result);
            } else if first_set != 'r' && second_set != 'r' {
                result = set_intersection(&variables[&first_set], &variables[&second_set]);
            }
            stack.push('r');
        } else if c == '!' {
            let first_set = stack.pop().unwrap();
            if first_set == 'r' {
                result = set_inverse(&super_set, &result);
            }
            // } else if first_set != 'r' {
            //     result = set_inverse(&super_set, &variables[&first_set]);
            // }
            stack.push('r');
        }
        if c.is_ascii_uppercase() {
            stack.push(c)
        }
    }
    result
}
pub fn test_set_eval() {
    let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];

    assert_eq!(eval_set("AB&", &sets), [0]);
    println!(
        " AB& WITH this set {:?}\n -> {:?}",
        sets,
        eval_set("AB&", &sets)
    );
    let sets = vec![vec![0, 1, 2], vec![3, 4, 5]];
    println!(
        " AB| WITH this set {:?}\n -> {:?}",
        sets,
        eval_set("AB|", &sets)
    );
    let mut result: Vec<i32> = eval_set("AB|", &sets);
    result.sort();
    assert_eq!(result, vec![0, 1, 2, 3, 4, 5]);

    let sets = vec![vec![0, 1, 2]];
    println!(
        " A! WITH this set {:?}\n -> {:?}",
        sets,
        eval_set("A!", &sets)
    );
    assert_eq!(eval_set("A!", &sets), []);
    let sets = vec![vec![0], vec![0], vec![]];
    println!(
        " ABC&& WITH this set {:?}\n -> {:?}",
        sets,
        eval_set("ABC&&", &sets)
    );
    assert_eq!(eval_set("ABC&&", &sets), []);
    let sets = vec![vec![0], vec![0], vec![0]];
    println!(
        " ABC|| WITH this set {:?}\n -> {:?}",
        sets,
        eval_set("ABC||", &sets)
    );
    assert_eq!(eval_set("ABC||", &sets), [0]);

    assert_eq!(eval_set("ABC&&", &sets), [0]);
    println!(
        " ABC&& WITH this set {:?}\n -> {:?}",
        sets,
        eval_set("ABC&&", &sets)
    );
    println!(
        " ABC^^ WITH this set {:?}\n -> {:?}",
        sets,
        eval_set("ABC^^", &sets)
    );
    assert_eq!(eval_set("ABC^^", &sets), [0]);
    println!(
        " ABC>> WITH this set {:?}\n -> {:?}",
        sets,
        eval_set("ABC>>", &sets)
    );
    assert_eq!(eval_set("ABC>>", &sets), [0]);

    let sets = vec![vec![], vec![42]];
    println!(
        " A! WITH this set {:?}\n -> {:?}",
        sets,
        eval_set("A!", &sets)
    );
    assert_eq!(eval_set("A!", &sets), [42]);
}
