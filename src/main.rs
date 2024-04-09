extern crate calamine as ca;


mod vals;
mod ios;
mod calcs;

use ca::{open_workbook, Xlsx};
use vals::Obj;

fn main() {
    let path = "test.xlsx";
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

    let obj = Obj::create(&mut workbook);
    
    for el in &obj.elements {
        println!(
            "{:?} \t B :{:?}  E :{:?}  \t PG : {:?}",
            el, obj.nodes[el.node_b_id], obj.nodes[el.node_e_id], obj.physgeos[el.phys_geo_id]
        );
    }
    for load in obj.loads {
        println!("{:?} \t N : {:?} ", load, obj.nodes[load.node_id]);
    }

    let ob_local_st = obj.elements[0].c_localc_st(&obj.physgeos);
    let ob_fr_cos =  obj.elements[0].c_cos_matrix();
    let ob_fr_cos_t = ob_fr_cos.transpose();
    print!("Stiffness matrix of the 1st element: {}",ob_local_st);
    print!("Cosine matrix of the 1st element: {}", ob_fr_cos);
    print!("Transposed cosine matrix of the 1st element: {}",ob_fr_cos_t);
    print!(
        "Global matrix of the 1st element: {}",
        (ob_fr_cos_t * ob_local_st) * ob_fr_cos
    );
}
