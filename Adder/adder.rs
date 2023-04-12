/**
  respecte la complexite O(1)
  si le temps d'exécution d'un algorithme dépend de la taille de l'entrée,
   sa complexité en temps n'est pas en O(1).
    Par exemple, si vous avez une fonction qui parcourt une liste d'éléments et qui effectue une opération sur chaque élément de la liste, 
    le temps d'exécution de cette fonction dépend de la taille de la liste et sa complexité en temps n'est pas en O(1).
*/
fn adder(a: u32, b: u32) -> u32
{
  let mut x = a;
  let mut y = b;
  let mut sum = a^ b;
  let mut carry = (a & b) << 1;
  if sum == 0
  {
    return carry;
  }
  // il n;est pas oblige de faire une boucle pour les petit npombre
  while carry != 0 { // on boucle sur le fait qu'il a des retenu donc on doit encore decaler
      sum = x ^ y;  // nous sert a avoir la retenu sans les bites , donc on a la premiere parti du calcul
      carry = (x & y) << 1; // ca nous permet d;avoir la retenu donc les retenu des bites que l'on a quand on les additionne car le AND & met a 1 que si c egal  c'est l'inverse du xor et on va le << 1 pour tout decaler pour avoir du coup la retenu
      x = sum;  // exemple avec 13 et 9 -> 4 le xor a enlever tout les bites pareil et a on ceux pas pareil
      y = carry; // and est de 9 >   mais ca correspond a nos retenu du coup il faut les decaler vers la gauche c a d les n ^ 2*1
  // on boucle jusqua temps qu'il n'a pas de retenu !! ( au niveau de l'addition des bites)
    }
    x = sum;
    return x;
}
fn main() {
println!(" resultat 10 + 10 = {} programme result {}",10 +10, adder(10,10));
println!(" resultat 0 + 1 = {} programme result {}",0 + 1, adder(0,1));
println!(" resultat 0 + 0 = {} programme result {}",0 +0, adder(0,0));
println!(" resultat 130 + 1490 = {} programme result {}",130 +1490, adder(130,1490));
println!(" resultat 83 + 49 = {} programme result {}",83+49, adder(83,49));

/***
*
    a^ b operation  sans les retenue  avec la table de verite ( c'est les retenu des bites);
      http://portail.lyc-la-martiniere-diderot.ac-lyon.fr/srv1/co/1_addition_bin.html
    */
}
