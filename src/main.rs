extern crate calamine as ca;

mod calcs;
mod ios;
mod vals;

use ca::{open_workbook, Xlsx};
use vals::Obj;

fn main() {
    let path = "test.xlsx";
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let obj = Obj::create(&mut workbook);

    print!("{}",obj.c_lvec().transpose() * obj.c_lvec());
    
}
