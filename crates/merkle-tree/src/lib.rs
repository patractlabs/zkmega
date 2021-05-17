#![no_std]

#[macro_use]
extern crate alloc;

#[allow(dead_code)]
mod merkle_tree;
mod mimc;

pub use self::mimc::*;
