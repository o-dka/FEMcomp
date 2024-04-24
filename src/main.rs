extern crate calamine as ca;
extern  crate nalgebra_sparse as nas;

pub mod calcs;
pub mod ios;
mod vals;

use ca::{open_workbook, Xlsx};
use vals::Obj;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1]; // INPUT YOUR OWN FILE 
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let mut obj = Obj::create(&mut workbook); 
    // print!("{}",&obj.c_glvec());
    print!("{}", &obj.c_gzvec());
    obj.c_s();
    obj.s.iter().for_each(|f| print!("{}",f));

}
