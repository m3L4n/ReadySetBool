/**
    respect la complexite car ne depend pas de la taille  du n nombre et sera constant  et effectue les mm operatons quoi qu'il arrive.
*/
fn adder(a: u32, b: u32) -> u32 {
    let mut x = a;
    let mut y = b;
    let mut sum = a ^ b;
    let mut carry = (a & b) << 1;

    //5  0000 0101
    //6  0000 0110
    // -> sum = 0000 0011 -> 3 ( a ^ b )
    // carry = 0000 0100   -> ( a & b ) et on decalle de 1 = 0000 1000 - > ca nous donne 8
    // sum = 0000 1011 - > 11
    // carry = 0000 00000 = 0 ;
    //carry = 0
    if x == 0 {
        return b;
    }
    if y == 0 {
        return a;
    }
    while carry != 0 {
        // on boucle sur le fait qu'il a des retenu
        sum = x ^ y; // nous sert a avoir la retenu sans les bites , donc on a la premiere parti du calcul
        carry = (x & y) << 1; // ca nous permet d;avoir la retenu donc les retenu des bites que l'on a quand on les additionne car le AND & met a 1 que si c egal  c'est l'inverse du xor et on va le << 1 pour tout decaler pour avoir du coup la retenu
        x = sum;
        y = carry;
    }
    x = sum;
    return x;
}
fn main() {
    println!(
        " resultat 10 + 10 = {} programme result {}",
        10 + 10,
        adder(10, 10)
    );
    println!(
        " resultat 0 + 1 = {} programme result {}",
        0 + 1,
        adder(0, 1)
    );
    println!(
        " resultat 0 + 0 = {} programme result {}",
        0 + 0,
        adder(0, 0)
    );
    println!(
        " resultat 130 + 1490 = {} programme result {}",
        130 + 1490,
        adder(130, 1490)
    );
    println!(
        " resultat 83 + 49 = {} programme result {}",
        83 + 49,
        adder(83, 49)
    );
    println!(
        " resultat 1083 + 10049 = {} programme result {}",
        1083 + 10049,
        adder(1083, 10049)
    );

    /***
    *
        a^ b operation  sans les retenue  avec la table de verite ( c'est les retenu des bites);
          http://portail.lyc-la-martiniere-diderot.ac-lyon.fr/srv1/co/1_addition_bin.html
        */
}
