use nalgebra::Vector6;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Node(
    pub(crate) f32, // x
    pub(crate) f32, // y
);

#[derive(Debug)]

pub(crate) struct Load {
    pub(crate) node_id: usize,
    pub(crate) forces: [f32;3],
}
#[derive(Debug)]
pub(crate) struct PhysGeo {
    pub(crate) a: f32, // this is actually an area
    pub(crate) j: f32, //
    pub(crate) e: f32, //
}
#[derive(Debug)]
pub(crate) struct Constraint {
    pub(crate) node_id: usize,
    pub(crate) stiffness: [f32;3],
}
#[derive(Debug)]
pub(crate) struct Element {
    pub(crate) b_id: usize, // beggining
    pub(crate) e_id: usize, // end
    pub(crate) phys_geo_id: usize,
    pub(crate) l: f32, // length
    pub(crate) element_sin: f32,
    pub(crate) element_cos: f32,
}
pub struct Obj {
    pub(crate) elements: Vec<Element>,
    pub(crate) nodes: Vec<Node>,
    pub(crate) loads: Vec<Load>,
    pub(crate) physgeos: Vec<PhysGeo>,
    pub(crate) constraints: Vec<Constraint>,
    pub s : Vec<Vector6<f32>>,
}
