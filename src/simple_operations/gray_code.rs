pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}
pub fn test_gray_code() {
    println!(" -> 0 = {}", gray_code(0));
    assert_eq!(gray_code(0), 0);
    println!(" -> 1 ={}", gray_code(1));
    assert_eq!(gray_code(1), 1);
    println!(" -> 2 ={}", gray_code(2));

    assert_eq!(gray_code(2), 3);
    // 3
    println!("-> 3 = {}", gray_code(3));
    assert_eq!(gray_code(3), 2);
    // 2
    println!("-> 4 = {}", gray_code(4));
    assert_eq!(gray_code(4), 6);
    // 6
    println!(" -> 5 = {}", gray_code(5));
    assert_eq!(gray_code(5), 7);
    // 7
    println!(" -> 6 = {}", gray_code(6));
    assert_eq!(gray_code(6), 5);
    // 5
    println!(" -> 7 = {}", gray_code(7));
    assert_eq!(gray_code(7), 4);
    println!(" -> 8 = {}", gray_code(8));
    assert_eq!(gray_code(8), 12);
    // 4
}
