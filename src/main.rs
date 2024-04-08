extern crate calamine as ca;
extern crate nalgebra as na;
extern crate nalgebra_sparse as na_sparse;


use na::{Matrix2, Matrix4, Matrix6};
// use na_sparse::CooMatrix;

use ca::{open_workbook, Data, DataType, Range, Reader, Xlsx};

#[derive(Debug, Clone, Copy)]
struct Node(f32, f32);

#[derive(Debug)]

struct Load {
    node_id: usize,
    forces: (f32, f32, f32),
}
#[derive(Debug)]
struct PhysGeo {
    f: f32,
    j: f32,
    e: f32,
}
struct Constrains {
    node_id: usize,
    dof: (bool, bool, bool),
}
#[derive(Debug)]
struct Element {
    node_b_id: usize,
    node_e_id: usize,
    phys_geo_id: usize,
    l: f32,
    element_sin: f32,
    element_cos: f32,
}

trait New {
    fn new(row: &[Data]) -> Self;
}

fn fill_anything<T: New>(s: Range<Data>, vec: &mut Vec<T>) {
    for row in s.rows() {
        vec.push(T::new(row));
    }
}
impl New for Node {
    fn new(row: &[Data]) -> Self {
        Node(
            row[0].get_float().unwrap() as f32,
            row[1].get_float().unwrap() as f32,
        )
    }
}
impl New for Load {
    fn new(row: &[Data]) -> Self {
        Load {
            node_id: row[0].get_float().unwrap() as usize,
            forces: (
                row[1].get_float().unwrap() as f32,
                row[2].get_float().unwrap() as f32,
                row[3].get_float().unwrap() as f32,
            ),
        }
    }
}
impl New for PhysGeo {
    fn new(row: &[Data]) -> Self {
        PhysGeo {
            f: row[0].get_float().unwrap() as f32,
            j: row[1].get_float().unwrap() as f32,
            e: row[2].get_float().unwrap() as f32,
        }
    }
}
impl New for Constrains {
    fn new(row: &[Data]) -> Self {
        Constrains {
            node_id: row[0].get_float().unwrap() as usize,
            dof: (
                row[1].get_float().unwrap() != 0.0,
                row[2].get_float().unwrap() != 0.0,
                row[3].get_float().unwrap() != 0.0,
            ),
        }
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
            (self.node_b_id, self.node_e_id),
            self.phys_geo_id,
            self.l,
            self.element_sin,
            self.element_cos
        )
    }
}
impl Element {
    pub fn create(row: &[Data], vec_of_nodes: &Vec<Node>) -> Self {
        let node_b_id = row[0].get_float().unwrap() as usize;
        let node_e_id = row[1].get_float().unwrap() as usize;
        let phys_geo_id = row[2].get_float().unwrap() as usize;
        let a = (vec_of_nodes[node_b_id].0 - vec_of_nodes[node_e_id].0).abs();
        let b = (vec_of_nodes[node_b_id].1 - vec_of_nodes[node_e_id].1).abs();
        let l = (a.powf(2.0) + b.powf(2.0)).sqrt();
        Element {
            node_b_id,
            node_e_id,
            phys_geo_id,
            l,
            element_sin: a / l,
            element_cos: b / l,
        }
    }
    fn c_rs_loc(&self, pgs: &Vec<PhysGeo>) -> [f32;4] {
        let e = pgs[self.phys_geo_id].e;
        let f = pgs[self.phys_geo_id].f;
        let r: [f32;4] = 
            [
                ((e * f) / self.l),
                -((e * f) / self.l),
                -((e * f) / self.l),
                ((e * f) / self.l),
            ]
            ;
        r
    }

    fn c_sz_loc(&self, pgs: &Vec<PhysGeo>) -> [f32;16] {
        let e: f32 = pgs[self.phys_geo_id].e;
        let j: f32 = pgs[self.phys_geo_id].j;
        let r:  [f32;16] = 
            [
                //  1st row
                ((12.0 * e * j) / self.l.powf(3.0)),
                ((6.0 * e * j) / self.l.powf(2.0)),
                -((12.0 * e * j) / self.l.powf(3.0)),
                ((6.0 * e * j) / self.l.powf(2.0)),
                // 2nd row
                ((6.0 * e * j) / self.l.powf(2.0)),
                ((4.0 * e * j) / self.l),
                -((6.0 * e * j) / self.l.powf(2.0)),
                ((2.0 * e * j) / self.l),
                // 3rd row
                -((12.0 * e * j) / self.l.powf(3.0)),
                -((6.0 * e * j) / self.l.powf(2.0)),
                ((12.0 * e * j) / self.l.powf(3.0)),
                -((6.0 * e * j) / self.l.powf(2.0)),
                // 4th row
                ((6.0 * e * j) / self.l.powf(2.0)),
                ((2.0 * e * j) / self.l),
                -((6.0 * e * j) / self.l.powf(2.0)),
                ((4.0 * e * j) / self.l),
            ];
        r
    }
    pub fn c_localc_st(&self, pgs: &Vec<PhysGeo>) -> Matrix6<f32> {
        let rs = self.c_rs_loc(pgs);
        let sz = self.c_sz_loc(pgs);
        let mut stiffness: Matrix6<f32> = Matrix6::from_iterator( 
            [
                rs[0], 0.0, 0.0, rs[1] ,0.0,0.0,
                0.0, sz[0],sz[1],0.0,sz[2],sz[3],
                0.0, sz[4],sz[5],0.0,sz[6],sz[7],
                rs[2], 0.0, 0.0, rs[3] ,0.0,0.0,
                0.0, sz[8],sz[9],0.0,sz[10],sz[11],
                0.0, sz[12],sz[13],0.0,sz[14],sz[15],
            ]
            .into_iter());
        stiffness
    }
}

fn main() {
    let path = "test.xlsx";
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

    let mut nodes: Vec<Node> = Vec::new();
    let mut loads: Vec<Load> = Vec::new();
    let mut test_obj: Vec<Element> = Vec::new();
    let mut physgeos: Vec<PhysGeo> = Vec::new();
    let mut constraints: Vec<Constrains> = Vec::new();

    for name in workbook.sheet_names() {
        match name.as_ref() {
            "nodes" => fill_anything(workbook.worksheet_range(&name).unwrap(), &mut nodes),
            "loads" => fill_anything(workbook.worksheet_range(&name).unwrap(), &mut loads),
            "properties" => fill_anything(workbook.worksheet_range(&name).unwrap(), &mut physgeos),
            "elements" => {
                if !nodes.is_empty() {
                    for row in workbook.worksheet_range(&name).unwrap().rows() {
                        test_obj.push(Element::create(row, &nodes))
                    }
                }
            }
            "constraints" => {
                fill_anything(workbook.worksheet_range(&name).unwrap(), &mut constraints)
            }

            _ => println!(
                "Unknown sheet name : {}, check the workbook for errors",
                name
            ),
        }
    }
    for obj in &test_obj {
        println!(
            "{:?} \t B :{:?}  E :{:?}  \t PG : {:?}",
            obj, nodes[obj.node_b_id], nodes[obj.node_e_id], physgeos[obj.phys_geo_id]
        );
    }
    for load in loads {
        println!("{:?} \t N : {:?} ", load, nodes[load.node_id]);
    }
    print!("Stiffness matrix of the 1st element: {}",test_obj[0].c_localc_st(&physgeos));
}
