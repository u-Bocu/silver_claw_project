#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

// Local module
//use silver_claw_lib::*;

// Test suites
mod circular_buffer_test_suite;
mod geometry_test_suite;

fn main() {
    println!("Execute circular_buffer_test_suite: ");
    match circular_buffer_test_suite::execute() {
        Ok(_) => {
            println!("\x1b[0;31mPASS\x1b[0m");
        }
        Err(_) => {
            println!("\x1b[0;31mFAIL\x1b[0m");
        }
    }

    println!("Execute geometry_test_suite: ");
    match geometry_test_suite::execute() {
        Ok(_) => {
            println!("\x1b[0;31mPASS\x1b[0m");
        }
        Err(_) => {
            println!("\x1b[0;31mFAIL\x1b[0m");
        }
    }
}
