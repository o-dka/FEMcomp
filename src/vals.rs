#[derive(Debug, Clone, Copy)]
pub struct Node(pub f32,  pub f32);

#[derive(Debug)]

pub struct Load {
    pub node_id: usize,
    pub forces: (f32, f32, f32),
}
#[derive(Debug)]
pub struct PhysGeo {
    pub f: f32,
    pub j: f32,
    pub e: f32,
}
pub struct Constrains {
    pub node_id: usize,
    pub dof: (bool, bool, bool),
}
#[derive(Debug)]
pub struct Element {
    pub node_b_id: usize,
    pub node_e_id: usize,
    pub phys_geo_id: usize,
    pub l: f32,
    pub element_sin: f32,
    pub element_cos: f32,
}

pub struct Obj {
  pub elements : Vec<Element>,
  pub nodes: Vec<Node> ,
  pub loads: Vec<Load>,
  pub physgeos: Vec<PhysGeo>,
  pub constraints: Vec<Constrains>,

}