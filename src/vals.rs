#[derive(Debug, Clone, Copy)]
pub(crate) struct Node(
    pub(crate) f32, // x
    pub(crate) f32, // y
);

#[derive(Debug)]

pub(crate) struct Load {
    pub(crate) node_id: usize,
    pub(crate) forces: (f32, f32, f32),
}
#[derive(Debug)]
pub(crate) struct PhysGeo {
    pub(crate) f: f32, // this is actually an area
    pub(crate) j: f32, //
    pub(crate) e: f32, //
}
pub(crate) struct Constrains {
    pub(crate) node_id: usize,
    pub(crate) dof: (bool, bool, bool),
}
#[derive(Debug)]
pub(crate) struct Element {
    pub(crate) node_b_id: usize, // beggining
    pub(crate) node_e_id: usize, // end
    pub(crate) phys_geo_id: usize,
    pub(crate) l: f32, // length
    pub(crate) element_sin: f32,
    pub(crate) element_cos: f32,
}
pub(crate) struct Obj {
    pub(crate) elements: Vec<Element>,
    pub(crate) nodes: Vec<Node>,
    pub(crate) loads: Vec<Load>,
    pub(crate) physgeos: Vec<PhysGeo>,
    pub(crate) constraints: Vec<Constrains>,
}
