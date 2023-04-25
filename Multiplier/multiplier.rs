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
  while carry != 0 { 
      sum = x ^ y; 
      carry = (x & y) << 1;
      x = sum; 
      y = carry;
    }
    x = sum;
    return x;
}

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
fn multiplier(a: u32, b: u32) -> u32{

  let mut result = 0;
  let mut x = a;
  let mut y = b;
  while y > 0 { //on boucle sur y 
    let z = y & 1; // permet de decider si on doit l'ajouter au produit si z = 0 alors ca ne sert a rien de l;ajouter si c'est impaire on le rajoute 
    if z == 1 
    {
      // ce qu'on rajoute du coup c;est sois mm car une multiplication c cb de fois te rajoutes dans le nbr et si on rajoute les nombre ca ne marche pas psk on mutiplie deja par deux dans chaque operations << , donc en evitant les nombre paire on divise par deux le nbr d'operation qu'il ya sur le nbr, donc le compte est bon on mutilipie par deux donc fqut qu;on divise par deux et donc pareil avec le nbr multiplieur
      result = adder(result, x); // tu ajoutes x au resultat  si z = 1 car cela veut dire qu;il a quelque chose a rajouter( des unites en l'occurence)
        }
     x = x << 1; // on monte les bytes d'1 cran donc un peu comme x *2
     y = y >> 1; // on descend les bytes d'1 etage donc n /2  
  }
  return result;

}

fn main(){
  println!(" 79 * 2  = {} | my multiplier = {}",79*2, multiplier(79, 2));
  println!(" 79 * 1  = {} |  my multiplier = {}",79*1, multiplier(79, 1));
  println!(" 1* 1  = {} |  my multiplier = {}",1*1, multiplier(1, 1));  
  println!(" 123* 14 =  {} |  my multiplier = {}",123*14, multiplier(123, 14));
  println!(" 14* 14 =  {} |  my multiplier = {}",14*14, multiplier(14, 14));
  println!(" 14* 0 =  {} |   my multiplier = {}",14*0, multiplier(14, 0));
}