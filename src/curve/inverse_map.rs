fn recuperation_bit_per_bit(x: &mut u16, y: &mut u16, map: &mut u32) {
    let n = 1;
    let mut number = 0;
    let max_iteration = 15;
    for _i in 0..=max_iteration {
        let tmpres = *map & n;
        let res1: u16 = (tmpres as u16) << number;
        *y |= res1;

        *map >>= 1;
        let tmpres1 = *map & n;
        let res2: u16 = (tmpres1 as u16) << number;
        *x |= res2;
        number += 1;
        *map >>= 1;
    }
}

fn reverse_map(n: f64) -> (u16, u16) {
    let mut x: u16 = 0;
    let mut y: u16 = 0;

    let mut map: u32 = (n * u32::MAX as f64) as u32;
    recuperation_bit_per_bit(&mut x, &mut y, &mut map);

    (x, y)
}

pub fn test_inverse_map() {
    println!("( 0)\n -> {:?}\n", reverse_map(0.));
    println!(
        "( 0.0000000004656612874161595)\n -> {:?}\n",
        reverse_map(0.0000000004656612874161595)
    );
    println!(
        "( 0.00000000023283064370807974 )\n -> {:?}\n",
        reverse_map(0.00000000023283064370807974)
    );
    println!(
        "(0.0000035874545582540926)\n -> {:?}\n",
        reverse_map(0.0000035874545582540926)
    );
    println!(
        "(0.002113968367249232 )\n -> {:?}\n",
        reverse_map(0.002113968367249232)
    );
    println!(
        "(0.0010569845328705816)\n -> {:?}\n",
        reverse_map(0.0010569845328705816)
    );
    println!(
        "(0.9999999993015081)\n -> {:?}\n",
        reverse_map(0.9999999993015081)
    );
    println!(
        "(0.9999999997671694)\n -> {:?}\n",
        reverse_map(0.9999999997671694)
    );

    println!("(1)\n -> {:?}", reverse_map(1.));
}
