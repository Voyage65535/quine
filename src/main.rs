#![feature(proc_macro_hygiene)]
use quine::quine;

fn main() {
    println!("{}", quine!());
}

