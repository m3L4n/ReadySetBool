
fn recuperation_bit_per_bit( x :&mut u16, y :&mut u16, map :& mut u32){

  let n = 1;
  let mut number = 0;
  let max_iteration = 15;
  for _i in 0..=max_iteration {
    let tmpres = *map & n;
    let res1 : u16 = (tmpres as u16 ) << number;
    *y |= res1;

    *map = *map >> 1;
    let tmpres1 = *map & n;
    let res2 : u16 = (tmpres1 as u16 ) << number;
    *x |= res2;
    number += 1;
    *map = *map >> 1;
  }
  
}



fn reverse_map(n: f64) -> (u16, u16){
let mut x : u16= 0; 
let mut y : u16 = 0;
 
let mut map : u32 = (n  * u32::MAX as f64) as u32;
recuperation_bit_per_bit(&mut x, &mut y, &mut map);

return (x, y);



}

fn main(){

  println!(" 0.000000012805685403944386 -> {:?}", reverse_map(0.000000012805685403944386)); //-> 5 ,7
  println!("/ 0.9999999997671694 ->  {:?}", reverse_map(0.9999999997671694)); // 65535, 65534)
  println!("/ 0.9999999995343387 ->  {:?}", reverse_map(0.9999999995343387)); // 65535, 65534)
}