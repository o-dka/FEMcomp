extern crate calamine as ca;
extern  crate nalgebra_sparse as nas;
use nas::factorization::CscCholesky;
mod calcs;
mod ios;
mod vals;

use ca::{open_workbook, Xlsx};
use vals::Obj;

fn main() {
    let path = "test.xlsx";
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let obj = Obj::create(&mut workbook);
    let decomp = CscCholesky::factor(&obj.c_glob()).unwrap();
    print!("{}",&obj.c_lvec());

    print!("{}", decomp.solve(&obj.c_lvec()));

}
