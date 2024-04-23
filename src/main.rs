extern crate calamine as ca;
extern  crate nalgebra_sparse as nas;

mod calcs;
mod ios;
mod vals;

use ca::{open_workbook, Xlsx};
use vals::Obj;

fn main() {
    let path = "test2.xlsx";
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let obj = Obj::create(&mut workbook);
    print!("{}",&obj.c_glvec());
    print!("{}", &obj.c_gzvec());
    // obj.c_pvec();
    print!("{:?}", &obj.s);

}
