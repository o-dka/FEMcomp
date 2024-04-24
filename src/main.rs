extern crate calamine as ca;
extern  crate nalgebra_sparse as nas;

pub mod calcs;
pub mod ios;
mod vals;

use ca::{open_workbook, Xlsx};
use vals::Obj;
use core::panic;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        panic!("No file provided in args")
    }
    let path = &args[1]; 
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let mut obj = Obj::create(&mut workbook); 
    print!("{}", &obj.c_gzvec());
    obj.c_s();
    obj.s.iter().for_each(|f| print!("{}",f));

}
