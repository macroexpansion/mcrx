#![allow(dead_code)]

use mcrx::{derive_builder::Builder, stable_sort::stable_sorted};

#[derive(Builder)]
#[stable_sorted]
struct A {
    a: i32,
    b: i32,
    c: i32,
}

fn main() {}
