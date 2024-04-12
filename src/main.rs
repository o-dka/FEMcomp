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
    println!("=============");
    for el in &obj.elements {
        println!(
            "{:?} \t B :{:?}  E :{:?}  \t PG : {:?}",
            el, obj.nodes[el.node_b_id], obj.nodes[el.node_e_id], obj.physgeos[el.phys_geo_id]
        );
    }
    for load in &obj.loads {
        println!("{:?} \t N : {:?} ", load, obj.nodes[load.node_id]);
    }
    println!("=============\n\n");
    obj.c_glob();
}
