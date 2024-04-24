extern crate calamine as ca;
extern  crate nalgebra_sparse as nas;

pub mod calcs;
pub mod ios;
mod vals;

use ca::{open_workbook, Xlsx};
use vals::Obj;

fn main() {
    let path = "test2.xlsx";
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let mut obj = Obj::create(&mut workbook);
    print!("{}", &obj.c_gzvec());
    obj.c_s();
    obj.s.iter().for_each(|f| print!("{}",f));

}
