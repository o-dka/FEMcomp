extern crate calamine as ca;
extern  crate nalgebra_sparse as nas;
extern crate rust_xlsxwriter as xlsx_writer;
pub mod calcs;
mod ios;
pub mod vals;
use ca::{open_workbook, Error};
use vals::Obj;

pub fn create_obj_from_xlsx(path : &str ) -> Result<Obj,Error>{
    let mut workbook= open_workbook(path).expect("ERROR with file :");
    Ok(Obj::create(&mut workbook).unwrap())
}


// TODO output to xlsx
// pub fn output_obj_to_xlsx(obj : &Obj) {}
