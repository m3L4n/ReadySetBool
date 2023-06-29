pub fn adder(a: u32, b: u32) -> u32 {
    let mut x = a;
    let mut y = b;
    let mut sum = x ^ y;
    let mut carry = (x & y) << 1;
    if x == 0 {
        return b;
    }
    if y == 0 {
        return a;
    }
    for _ in 0..32 {
        if carry == 0 {
            break;
        }
        sum = x ^ y; // nous sert a avoir la retenu sans les bites , donc on a la premiere parti du calcul
        carry = (x & y) << 1; // ca nous permet d;avoir la retenu donc les retenu des bites que l'on a quand on les additionne car le AND & met a 1 que si c egal  c'est l'inverse du xor et on va le << 1 pour tout decaler pour avoir du coup la retenu
        x = sum;
        y = carry;
    }
    x = sum;
    x
}

pub fn test_adder() {
    println!(
        " resultat 10 + 10 = {} programme result {}",
        10 + 10,
        adder(10, 10)
    );
    assert_eq!(adder(10, 10), 20);
    println!(
        " resultat 0 + 1 = {} programme result {}",
        1,
        adder(0, 1)
    );
    assert_eq!(adder(0, 1), 1);
    println!(
        " resultat 2 + 2 = {} programme result {}",
        2 + 2,
        adder(2, 2)
    );
    assert_eq!(adder(2, 2), 4);
    println!(
        " resultat 0 + 0 = {} programme result {}",
        0,
        adder(0, 0)
    );
    assert_eq!(adder(0, 0), 0);
    println!(
        " resultat 130 + 1490 = {} programme result {}",
        130 + 1490,
        adder(130, 1490)
    );
    assert_eq!(adder(130, 1490), 1620);
    println!(
        " resultat 83 + 49 = {} programme result {}",
        83 + 49,
        adder(83, 49)
    );
    assert_eq!(adder(83, 49), 132);
    println!(
        " resultat 1083 + 10049 = {} programme result {}",
        1083 + 10049,
        adder(1083, 10049)
    );
    assert_eq!(adder(1083, 10049), 11132);
}
