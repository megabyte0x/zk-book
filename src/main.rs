// Converting Arthematic Circuits to R1CS

use ndarray::prelude::*;

/**
 * 
 * This is verifing the following constraint:
 * z === x * y
 */
pub fn z_equal_to_x_and_y() {
    let o = array![[0, 1, 0, 0]];
    let l = array![[0, 0, 1, 0]];
    let r = array![[0, 0, 0, 1]];

    let a = array![1, 4223, 41, 103];

    let r1 = l.dot(&a);
    let r2 = r.dot(&a);
    let r3 = o.dot(&a);

    let final_r = r1.dot(&r2);
    let final_l = r3[0];

    assert_eq!(final_l, final_r);

    println!("Z equal to X AND Y is Verified");
}

fn main() {
    z_equal_to_x_and_y();
}
