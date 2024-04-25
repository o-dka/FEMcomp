extern crate calamine as ca;
extern  crate nalgebra_sparse as nas;

pub mod calcs;
pub mod ios;
pub mod vals;
use ca::{open_workbook, Error};
use vals::Obj;

pub fn create_obj_from_xlsx(path : &str ) -> Result<Obj,Error>{
    let mut workbook= open_workbook(path).expect("ERROR with file :");
    Ok(Obj::create(&mut workbook))
}