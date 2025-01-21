# Multiple Mutable Borrows in Rust
This example demonstrates the common error of creating multiple mutable borrows of the same variable in Rust.  Rust's borrow checker prevents this to ensure memory safety. 

## The Bug
The `bug.rs` file shows a simple program that attempts to create two mutable references (`y` and `z`) to the same variable `x`.  This violates Rust's borrowing rules, resulting in a compile-time error. 

## The Solution
The `bugSolution.rs` demonstrates ways to solve this problem, which are refactoring the code to avoid multiple mutable borrows by using a single mutable reference or using interior mutability.