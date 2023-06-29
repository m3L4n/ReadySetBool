use crate::simple_operations::adder::adder;

/*
*http://fabrice.sincere.pagesperso-orange.fr/cm_electronique/electronique_numerique20/annexe/conversion_decimal_hexa02.pdf
tableau pour mieux comprendre comment ca se passe
*/
//x  0000 0010 (=2)
//y  0000 0011 (=3)
//z  0000 0001 (=1 y & 1)
//result = 0 + (2) = 000 0010 (2)
// x 0000 0100 (4)
// y 0000 0001 (1)
// z 0000 0001 (1 y & 1)
// result 2+ 4 = 000 0110 (6)

fn multiplier(a: u32, b: u32) -> u32 {
    let mut result = 0;
    let mut x = a;
    let mut y = b;
    for _ in 0..32 {
        if y == 0 {
            break;
        }
        //on boucle sur y
        let z = y & 1; // permet de decider si on doit l'ajouter au produit si z = 0 alors ca ne sert a rien de l;ajouter si c'est impaire on le rajoute
        if z == 1 {
            // ce qu'on rajoute du coup c;est sois mm car une multiplication c cb de fois te rajoutes dans le nbr et si on rajoute les nombre ca ne marche pas psk on mutiplie deja par deux dans chaque operations << , donc en evitant les nombre paire on divise par deux le nbr d'operation qu'il ya sur le nbr, donc le compte est bon on mutilipie par deux donc fqut qu;on divise par deux et donc pareil avec le nbr multiplieur
            result = adder(result, x); // tu ajoutes x au resultat  si z = 1 car cela veut dire qu;il a quelque chose a rajouter( des unites en l'occurence)
        }
        x <<= 1; // on monte les bytes d'1 cran donc un peu comme x *2
        y >>= 1; // on descend les bytes d'1 etage donc n /2
    }
    result
}

pub fn test_multiplier() {
    println!(
        " resultat 10 * 10 = {} programme result {}",
        10 * 10,
        multiplier(10, 10)
    );
    assert_eq!(multiplier(10, 10), 100);
    println!(
        " resultat 0 * 1 = {} programme result {}",
        0,
        multiplier(0, 1)
    );
    assert_eq!(multiplier(0, 1), 0);
    println!(
        " resultat 2 * 2 = {} programme result {}",
        2 * 2,
        multiplier(2, 2)
    );
    assert_eq!(multiplier(2, 2), 4);
    println!(
        " resultat 0 * 0 = {} programme result {}",
        0 * 0,
        multiplier(0, 0)
    );
    assert_eq!(multiplier(0, 0), 0);
    println!(
        " resultat 130 * 1490 = {} programme result {}",
        130 * 1490,
        multiplier(130, 1490)
    );
    assert_eq!(multiplier(130, 1490), 193700);
    println!(
        " resultat 83 * 49 = {} programme result {}",
        83 * 49,
        multiplier(83, 49)
    );
    assert_eq!(multiplier(83, 49), 4067);
    println!(
        " resultat 1083 * 10049 = {} programme result {}",
        1083 * 10049,
        multiplier(1083, 10049)
    );
}
