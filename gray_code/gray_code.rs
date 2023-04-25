

/***
https://fr.wikipedia.org/wiki/Code_de_Gray
-->  est un type de codage binaire permettant de ne modifier qu'un seul bit à la fois quand un nombre est augmenté d'une unité
a différence principale entre les deux est le fait que le codage de Gray de deux nombres consécutifs ne diffère que d'une position.
on divise le nbr par deux et on va tchek XOR davec le nbr ( 1 et 1 = 0 et 1 0 = 1) et cela nous la bonne solution
*/
// 0000 0111 ( 7 )
//  7 >> 1 = 0000 0011
//   result =  0000 0100 - > 4

fn gray_code(n: u32) -> u32
{
  return n ^ (n >> 1); 
}

fn main(){
println!("{}", gray_code(0));
println!("{}", gray_code(1));
println!("{}", gray_code(2));
// 3
println!("{}", gray_code(3));
// 2
println!("{}", gray_code(4));
// 6
println!("{}", gray_code(5));
// 7
println!("{}", gray_code(6));
// 5
println!("{}", gray_code(7));
// 4
println!("{}", gray_code(8));
// 12
}