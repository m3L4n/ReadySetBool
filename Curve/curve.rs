
fn add_bit_to_flat(x: u16, y:u16, result : &mut u32){

  let max_iteration = 15;
  let mut n = 1;
  let mut n1 = 1;
  let mut number = 0;
  // x = 0000 0001
  // y = 0000 0010
  for _i in 0..=max_iteration {
    let tmpres = x & n;
    let tmpres1 = y & n1;
    let res1 : u32 = (tmpres1 as u32)  << number;
    number += 1;
    let res2 : u32 = (tmpres as u32 ) << number;

    let restmp : u32= res2 | res1;
    *result |= restmp;
    n = n << 1;
    n1 = n1 << 1;

  }


}
fn map(x: u16, y: u16) -> f64{
  let mut result = 0;

 add_bit_to_flat(x, y, &mut result);
  return result as f64 / u32::MAX as f64;
}

fn main(){

  println!("{}", map(65535, 65534));
  println!("{}", map(65534, 65535));
  println!("{}", map(5, 7));
}