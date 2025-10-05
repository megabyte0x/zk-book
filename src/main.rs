// ZK proof from R1CS

use ndarray::prelude::*;

/**
 *
 * This is verifing the following constraint:
 * z === x * y
 */
pub fn z_equal_to_x_and_y() {
    // Output Matrix
    let o = array![[0, 1, 0, 0]];
    // Left hand side
    let l = array![[0, 0, 1, 0]];
    // Right hand side
    let r = array![[0, 0, 0, 1]];

    // Witness
    // [1, z, x, y]
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
