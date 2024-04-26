use nalgebra::Vector6;

#[derive(Debug, Clone, Copy)]
pub struct Node(
    pub f32, // x
    pub f32, // y
);

#[derive(Debug)]

pub struct Load {
    pub node_id: usize,
    pub forces: [f32;3],
}
#[derive(Debug)]
pub struct PhysGeo {
    pub(crate) a: f32, 
    pub(crate) j: f32, 
    pub(crate) e: f32, 
}
#[derive(Debug)]
pub struct Constraint {
    pub node_id: usize,
    pub stiffness: [f32;3],
}
#[derive(Debug)]
pub struct Element {
    pub b_id: usize, // beggining
    pub e_id: usize, // end
    pub phys_geo_id: usize,
    pub(crate) l: f32, // length
    pub(crate) element_sin: f32,
    pub(crate)  element_cos: f32,
}
#[derive(Debug)]
pub struct Obj {
    pub elements: Vec<Element>,
    pub nodes: Vec<Node>,
    pub loads: Vec<Load>,
    pub physgeos: Vec<PhysGeo>,
    pub constraints: Vec<Constraint>,
    pub s : Vec<Vector6<f32>>,
}
