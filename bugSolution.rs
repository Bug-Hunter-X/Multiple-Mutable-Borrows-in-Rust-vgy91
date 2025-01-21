fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10;
    println!("x = {}", x); // x = 10
}

//alternative solution using interior mutability
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    let y = x.borrow_mut();
    *y = 10;
    let z = x.borrow_mut();
    *z = 100;
    println!("x = {}", x.borrow());
}