mod common;
mod ownership;

use common::*;
use crate::ownership::*;

fn main() {
    let a: u8 = 123;
    let is_delete: bool = true;

    let array: [u8; 10] = [12, 21, 22, 23, 24, 25, 26, 27, 28, 29];
    println!("{}_{}", a, is_delete);
    println!("first: {}_ last: {}", array[0], array[2]);
    println!("sum: {}", use_fn(4, 2));
    println!("is_odd: {}", use_return(3));
    use_for();
    use_while();
    use_loop();
    use_loop_index();
    use_ownership();
    use_fn_ownership();
}