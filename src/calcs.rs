extern crate nalgebra as na;
extern crate nalgebra_sparse as na_sparse;

use crate::vals::{Element, Obj, PhysGeo};

use na::{DVector, Matrix6};
use na_sparse::{convert::serial::convert_coo_csc, CooMatrix, CscMatrix};

impl Element {
    fn c_localc_st(&self, pgs: &[PhysGeo]) -> Matrix6<f32> {
        let e: f32 = pgs[self.phys_geo_id].e;
        let j: f32 = pgs[self.phys_geo_id].j;
        let f = pgs[self.phys_geo_id].f;

        let rs = [
            ((e * f) / self.l),
            -((e * f) / self.l),
            -((e * f) / self.l),
            ((e * f) / self.l),
        ];

        let sz: [f32; 16] = [
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

        Matrix6::<f32>::from_iterator([
            rs[0], 0.0, 0.0, rs[1], 0.0, 0.0, 0.0, sz[0], sz[1], 0.0, sz[2], sz[3], 0.0, sz[4],
            sz[5], 0.0, sz[6], sz[7], rs[2], 0.0, 0.0, rs[3], 0.0, 0.0, 0.0, sz[8], sz[9], 0.0,
            sz[10], sz[11], 0.0, sz[12], sz[13], 0.0, sz[14], sz[15],
        ])
    }
    pub(crate) fn c_cos_matrix(&self) -> Matrix6<f32> {
        Matrix6::<f32>::from_row_iterator([
            //  1st row
            self.element_cos,
            self.element_sin,
            0.0,
            0.0,
            0.0,
            0.0,
            // 2nd row
            -1.0 * self.element_sin,
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
            -1.0 * self.element_sin,
            self.element_cos,
            0.0,
            // 6th row
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }
    pub(crate) fn c_glob_st(&self, pgs: &[PhysGeo]) -> Matrix6<f32> {
        (self.c_cos_matrix().transpose() * self.c_localc_st(pgs)) * self.c_cos_matrix()
    }
}

impl Obj {
    pub fn c_glob(&self) -> CscMatrix<f32> {
        let mut result: CooMatrix<f32> =
            CooMatrix::zeros((self.elements.len() * 3) +3, (self.elements.len() * 3) + 3 );

        for el in &self.elements {
            let el_r: Matrix6<f32> = el.c_glob_st(&self.physgeos);

            let el_enode_3 = el.node_e_id * 3;
            let el_bnode_3 = el.node_b_id * 3;
            (0..3).for_each(|i| {
                (0..3).for_each(|j| {
                    result.push(el_bnode_3 + j, el_bnode_3 + i, el_r[(i, j)]);
                    result.push(el_bnode_3 + j, el_enode_3 + i, el_r[((i + 3), j)]);
                    result.push(el_enode_3 + j, el_bnode_3 + i, el_r[(i, (j + 3))]);
                    result.push(el_enode_3 + j, el_enode_3 + i, el_r[((i + 3), (j + 3))]);

                    /*
                    alt implementation :
                    result.push(el_bnode_3 + j, el_bnode_3 + i, el_r[i * 6 + j]);
                    result.push(el_bnode_3 + j, el_enode_3 + i, el_r[(i + 3) * 6 + j]);
                    result.push(el_enode_3 + j, el_bnode_3 + i, el_r[i * 6 + j + 3]);
                    result.push(el_enode_3 + j, el_enode_3 + i, el_r[(i + 3) * 6 + j + 3]);
                    */
                });
            });
        }
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
    
    pub fn c_lvec(&self) -> DVector<f32>{
        let mut input: Vec<f32> = (0..(&self.elements.len() * 3)+3).map(|_x| 0.0).collect();

        self.loads.iter().for_each(|load| {
            self.constraints.iter().for_each(|cons| {
                match cons.node_id == load.node_id {
                    true => (),
                    false => (0..3).for_each(|f| {
                        println!("{} : {}",f,load.node_id);
                        input[load.node_id  + (f + 3)] = load.forces[f]; // :D
                    }),
                }
            });
        });
      
        DVector::<f32>::from_iterator(self.elements.len() * 3+3,input)
    }
}
