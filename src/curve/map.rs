//https://www.researchgate.net/figure/Morton-space-filling-curve-for-an-8-8-matrix-The-interactions-are-sorted-according-to_fig2_48304794
//https://en.wikipedia.org/wiki/Z-order_curve
fn add_bit_to_flat(x: u16, y: u16, result: &mut u32) {
    let max_iteration = 15;
    let mut n = 1;
    let mut n1 = 1;
    let mut number = 0;
    // x = 0000 0001
    // y = 0000 0010
    for _i in 0..=max_iteration {
        let tmpres = x & n;
        let tmpres1 = y & n1;
        let res1: u32 = (tmpres1 as u32) << number;
        number += 1;
        let res2: u32 = (tmpres as u32) << number;

        let restmp: u32 = res2 | res1;
        *result |= restmp;
        n <<= 1;
        n1 <<= 1;
    }
}
fn map(x: u16, y: u16) -> f64 {
    let mut result = 0;
    add_bit_to_flat(x, y, &mut result);
    result as f64 / u32::MAX as f64
}

pub fn test_map() {
    println!("( 0 et 0 )\n -> {}", map(0, 0));
    assert_eq!(map(0, 0), 0.);

    println!("( 1 et 0 )\n -> {}", map(1, 0));
    assert_eq!(map(1, 0), 0.0000000004656612874161595);

    println!("( 0 et 1 )\n -> {}", map(0, 1));
    assert_eq!(map(0, 1), 0.00000000023283064370807974);

    println!("( 100 et 100 )\n -> {}", map(100, 100));
    assert_eq!(map(100, 100), 0.0000035874545582540926);

    println!("( 30000 et 1 )\n -> {}", map(3000, 1));
    assert_eq!(map(3000, 1), 0.002113968367249232);

    println!("( 1 et 30000 )\n -> {}", map(1, 3000));
    assert_eq!(map(1, 3000), 0.0010569845328705816);

    println!("( 65534 et 65534 )\n -> {}", map(65534, 65534));
    assert_eq!(map(65534, 65534), 0.9999999993015081);

    println!("( 65535 et 65534 )\n -> {}", map(65535, 65534));
    assert_eq!(map(65535, 65534), 0.9999999997671694);

    println!("( 65535 et 65535 )\n -> {}", map(65535, 65535));
    assert_eq!(map(65535, 65535), 1.);
}
