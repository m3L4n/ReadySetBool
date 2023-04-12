/**
le powerset d'un ensemble donné est l'ensemble de tous les sous-ensembles possibles de cet ensemble,
 y compris l'ensemble vide et l'ensemble lui-même.
le powerset d'un ensemble A est un ensemble P qui contient tous les sous-ensembles possibles de A.
*/

fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
  let mut result = vec![vec![]];
  for &element in set {
      let mut temp = vec![];
      for subset in &result {
          let mut new_subset = subset.clone();
          new_subset.push(element);
          temp.push(new_subset);
      }
      result.append(&mut temp);
  }
  result
}

fn main() {
  let set = [1, 2, 3, 4, 5 , 6 , 7];
  let set1 = [1, 2, 3];
  let set2 = [1, 2, 3, 4, 5];

  println!(" vec :{:?} len: {}", powerset(&set),  powerset(&set).len());
  println!(" vec :{:?} len: {}", powerset(&set1),  powerset(&set1).len());
  println!(" vec :{:?} len: {}", powerset(&set2),  powerset(&set2).len());
}