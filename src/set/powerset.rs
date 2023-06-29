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

pub fn test_powerset() {
    println!(
        " vec :{:?} len: {}",
        powerset(&[1, 2, 3]),
        powerset(&[1, 2, 3]).len()
    );
    assert_eq!(
        powerset(&[1, 2, 3]),
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3]
        ]
    );
    println!(" vec :{:?} len: {}", powerset(&[]), powerset(&[]).len());
    assert_eq!(powerset(&[]), [[]]);
    println!(
        " vec :{:?} len: {}",
        powerset(&[1, 2]),
        powerset(&[1, 2]).len()
    );
    assert_eq!(
        powerset(&[1, 2]),
        vec![vec![], vec![1], vec![2], vec![1, 2]]
    );
    println!(" vec :{:?} len: {}", powerset(&[42]), powerset(&[42]).len());
    assert_eq!(powerset(&[42]), vec![vec![], vec![42]]);
}
