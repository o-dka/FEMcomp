extern crate nalgebra as na;
extern crate nalgebra_sparse as na_sparse;

use crate::vals::{Element, Obj, PhysGeo};

use na::{DVector, Matrix6, Vector6};
use na_sparse::{
    convert::serial::convert_coo_csc, factorization::CscCholesky, CooMatrix, CscMatrix,
};

const CM: Matrix6<f32> = Matrix6::<f32>::new(
    -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
);

impl Element {
    fn c_localc_st(&self, pgs: &[PhysGeo]) -> Matrix6<f32> {
        let l: f32 = self.l;
        if pgs.is_empty() {
            panic!("Physgeo vector is empty!");
        }
        let e: f32 = pgs[self.phys_geo_id].e;
        let j: f32 = pgs[self.phys_geo_id].j;
        let a: f32 = pgs[self.phys_geo_id].a;

        let rs: f32 = (e * a) / l;
        let sz: [f32; 4] = [
            ((12.0 * e * j) / l.powf(3.0)),
            ((6.0 * e * j) / l.powf(2.0)),
            ((4.0 * e * j) / l),
            ((2.0 * e * j) / l),
        ];

        Matrix6::<f32>::new(
            rs, 0.0, 0.0, -rs, 0.0, 0.0, // 1
            0.0, sz[0], sz[1], 0.0, -sz[0], sz[1], // 2
            0.0, sz[1], sz[2], 0.0, -sz[1], sz[3], // 3
            -rs, 0.0, 0.0, rs, 0.0, 0.0, // 4
            0.0, -sz[0], -sz[1], 0.0, sz[0], -sz[1], // 5
            0.0, sz[1], sz[3], 0.0, -sz[1], sz[2], // 6
        )
    }
    fn c_cos_matrix(&self) -> Matrix6<f32> {
        Matrix6::<f32>::new(
            //  1st row
            self.element_cos,
            self.element_sin,
            0.0,
            0.0,
            0.0,
            0.0,
            // 2nd row
            -self.element_sin,
            self.element_cos,
            0.0,
            0.0,
            0.0,
            0.0,
            // 3rd row
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            // 4th row
            0.0,
            0.0,
            0.0,
            self.element_cos,
            self.element_sin,
            0.0,
            // 5th row
            0.0,
            0.0,
            0.0,
            -self.element_sin,
            self.element_cos,
            0.0,
            // 6th row
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }
    fn c_glob_st(&self, pgs: &[PhysGeo]) -> Matrix6<f32> {
        (self.c_cos_matrix().transpose() * self.c_localc_st(pgs)) * self.c_cos_matrix()
    }
}

impl Obj {
    pub fn c_glob(&self) -> CscMatrix<f32> {
        let mut result: CooMatrix<f32> =
            CooMatrix::zeros((self.elements.len() * 3) + 3, (self.elements.len() * 3) + 3);

        self.elements.iter().for_each(|el| {
            let el_r: Matrix6<f32> = el.c_glob_st(&self.physgeos);

            let el_enode_3: usize = el.e_id * 3;
            let el_bnode_3: usize = el.b_id * 3;

            (0..3).for_each(|i| {
                (0..3).for_each(|j| {
                    result.push(el_bnode_3 + j, el_bnode_3 + i, el_r[(i, j)]);
                    result.push(el_bnode_3 + j, el_enode_3 + i, el_r[((i + 3), j)]);
                    result.push(el_enode_3 + j, el_bnode_3 + i, el_r[(i, (j + 3))]);
                    result.push(el_enode_3 + j, el_enode_3 + i, el_r[((i + 3), (j + 3))]);
                });
            });
        });

        // le epic kostyl , removes the duplicates
        result = CooMatrix::from(&convert_coo_csc(&result));

        self.constraints.iter().for_each(|cnt| {
            result.triplet_iter_mut().for_each(|x| {
                cnt.stiffness.iter().enumerate().for_each(|(id, &dof)| {
                    if dof > 0.0 {
                        let indexer = cnt.node_id * 3 + id;
                        match (indexer == x.0, indexer == x.1) {
                            (true, true) => *x.2 = 1.0,
                            (true, false) => *x.2 = 0.0,
                            (false, true) => *x.2 = 0.0,
                            _ => (), // dont delete this line
                        }
                    }
                });
            });
        });
        CscMatrix::from(&result)
    }
    fn c_glvec(&self) -> DVector<f32> {
        let mut input: Vec<f32> = (0..(&self.elements.len() * 3) + 3).map(|_x| 0.0).collect();
        for load in self.loads.iter() {
            for cons in self.constraints.iter() {
                match cons.node_id == load.node_id {
                    true => continue,
                    false => {
                        (0..3).for_each(|f| {
                            input[load.node_id * 3 + f] = load.forces[f];
                        });
                    }
                }
            }
        }
        DVector::<f32>::from_vec(input)
    }
    pub fn c_gzvec(&self) -> DVector<f32> {
        match CscCholesky::factor(&self.c_glob()) {
            Ok(matrix) => {
                DVector::<f32>::from_vec(matrix.solve(&self.c_glvec()).iter().cloned().collect())
            }
            Err(err) => {
                println!("{}", err);
                DVector::<f32>::zeros(0)
            }
        }
    }
    pub fn c_s(&mut self) {
        let z_vec = self.c_gzvec();
        if z_vec.is_empty() {
            println!("Z vector is empty");
        } else {
            for el in self.elements.iter() {
                let z_loc = Vector6::<f32>::new(
                    z_vec[el.b_id * 3],
                    z_vec[el.b_id * 3 + 1],
                    z_vec[el.b_id * 3 + 2],
                    z_vec[el.e_id * 3],
                    z_vec[el.e_id * 3 + 1],
                    z_vec[el.e_id * 3 + 2],
                );

                let x = el.c_localc_st(&self.physgeos).transpose()
                    * el.c_cos_matrix().transpose()
                    * z_loc;
                self.s.push(CM.transpose() * x); //
            }
        }
    }
}
