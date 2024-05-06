use xlsx_writer::*;
use std::{fs:: File, io::BufReader};

use calamine::{Data, DataType, Range, Reader, Xlsx};
use nalgebra::Vector6;

use crate::vals::{Constraint, Element, Load, Node, Obj, PhysGeo};

fn check_workbook(workbook: &mut Xlsx<BufReader<File>>) -> bool {
    !(workbook.sheet_names().is_empty()
        || workbook
            .sheet_names()
            .into_iter()
            .any(|name| workbook.worksheet_range(&name).unwrap().is_empty()))
}
trait New {
    fn new(row: &[Data]) -> Self;
}
fn fill_anything<T: New>(s: Range<Data>, vec: &mut Vec<T>) {
    for row in s.rows().skip(1) {
        vec.push(T::new(row));
    }
}

impl New for Node {
    fn new(row: &[Data]) -> Self {
        Node(
            row[0].get_float().unwrap() as f32, // x
            row[1].get_float().unwrap() as f32, // y
        )
    }
}
impl New for Load {
    fn new(row: &[Data]) -> Self {
        Load {
            node_id: row[0].get_float().unwrap() as usize,
            forces: [
                row[1].get_float().unwrap() as f32,
                row[2].get_float().unwrap() as f32,
                row[3].get_float().unwrap() as f32,
            ],
        }
    }
}
impl New for PhysGeo {
    fn new(row: &[Data]) -> Self {
        PhysGeo {
            a: row[0].get_float().unwrap() as f32,
            j: row[1].get_float().unwrap() as f32,
            e: row[2].get_float().unwrap() as f32,
        }
    }
}
impl New for Constraint {
    fn new(row: &[Data]) -> Self {
        Constraint {
            node_id: row[0].get_float().unwrap() as usize,
            stiffness: [
                row[1].get_float().unwrap() as f32,
                row[2].get_float().unwrap() as f32,
                row[3].get_float().unwrap() as f32,
            ],
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"x : {} y : {}", self.0, self.1)
    }
}
impl std::fmt::Display for Load {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node number : {} ,Loads : \nx {} ;\ny {} ;\np {} ;\n",self.node_id, self.forces[0],self.forces[1],self.forces[2])
    }
}
impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node number : {} , Constraints : \nx {} ;\ny {} ;\np {} ;\n",self.node_id, self.stiffness[0],self.stiffness[1],self.stiffness[2])
    }
}
impl std::fmt::Display for PhysGeo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PhysGeo : \na {} ;\nj {} ;\ne {} ;\n",self.a,self.j,self.e)
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "points : {:?} 
          \nvalues : {:?} 
          \nlength: {:?} 
          \nsin : {:?} 
          \ncos: {:?}",
            (self.b_id, self.e_id),
            self.phys_geo_id,
            self.l,
            self.element_sin,
            self.element_cos
        )
    }
}

impl Element {
    fn create(row: &[Data], vec_of_nodes: &[Node]) -> Self {
        let b_id = row[0].get_float().unwrap() as usize;
        let e_id = row[1].get_float().unwrap() as usize;
        let phys_geo_id = row[2].get_float().unwrap() as usize;
        let dx = vec_of_nodes[b_id].0 - vec_of_nodes[e_id].0;
        let dy = vec_of_nodes[b_id].1 - vec_of_nodes[e_id].1;
        let l = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
        Element {
            b_id,
            e_id,
            phys_geo_id,
            l,
            element_sin: dy / l,
            element_cos: dx / l,
        }
    }
}

impl std::fmt::Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "nodes : {:#?} 
          \nloads: {:#?} 
          \nphysgeos : {:#?} 
          \nconstraints: {:#?},
          \n s vector: {:#?}",
            self.nodes, self.loads, self.physgeos, self.constraints, self.s
        )
    }
}

impl Obj {
    pub fn create(workbook: &mut Xlsx<BufReader<File>>) -> std::option::Option<Self> {
        if check_workbook(workbook) {
            let mut nodes: Vec<Node> = Vec::new();
            let mut loads: Vec<Load> = Vec::new();
            let mut elements: Vec<Element> = Vec::new();
            let mut physgeos: Vec<PhysGeo> = Vec::new();
            let mut constraints: Vec<Constraint> = Vec::new();
            for name in workbook.sheet_names().into_iter() {
                match name.as_ref() {
                    "nodes" => fill_anything(workbook.worksheet_range(&name).unwrap(), &mut nodes),
                    "loads" => fill_anything(workbook.worksheet_range(&name).unwrap(), &mut loads),
                    "properties" => {
                        fill_anything(workbook.worksheet_range(&name).unwrap(), &mut physgeos)
                    }
                    "elements" => match nodes.is_empty() {
                        true => panic!("Sheets not in proper order, nodes should always be the first sheet!"),
                        false => {
                            for row in workbook.worksheet_range(&name).unwrap().rows().skip(1) {
                                elements.push(Element::create(row, &nodes))
                            }
                        }
                    },
                    "constraints" => {
                        fill_anything(workbook.worksheet_range(&name).unwrap(), &mut constraints)
                    }

                    _ => println!("{} is not a recognizible sheet name", name),
                }
            }
            let s = Vec::<Vector6<f32>>::new();

            Some(Obj {
                elements,
                nodes,
                loads,
                s,
                physgeos,
                constraints,
            })
        } else {
            panic!("Sheets missing!!");
        }
    }

    pub fn create_empty_obj() -> Self {
        Obj {
            elements: Vec::new(),
            nodes: Vec::new(),
            loads: Vec::new(),
            physgeos: Vec::new(),
            constraints: Vec::new(),
            s: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
 // Remove S vector output , name it after the file name 
    pub fn write_data(&self,obj_name : &str) -> Result<(), XlsxError> {
        let mut workbook = Workbook::new();
        let decimal_format = Format::new().set_num_format("0.000");
        
        let worksheet= workbook.add_worksheet().set_name("Z vector")?;
        worksheet.write(0, 0, "Z")?;
        self.c_gzvec().iter().enumerate().for_each(|somethng| {
            worksheet.write_with_format((somethng.0 as u32)+1, 0, *somethng.1, &decimal_format).unwrap();
        });
        // let worksheet = workbook.add_worksheet().set_name("S vectors")?;
        // worksheet.write(0, 0, "S")?;    
        // worksheet.write(0, 1, "Element")?;    
        // for (element_id, x) in self.s.iter().enumerate() {
        //     worksheet.write((element_id as u32) + 5, 1 , element_id as u32)?;
        //     for num in x.iter().enumerate() {
        //         worksheet.write_with_format((num.0 as u32)+1, 0, *num.1, &decimal_format).unwrap();
        //     }
        // }
        workbook.save(format!("./{} - output.xlsx" , obj_name))?;
        Ok(())
    }
}
