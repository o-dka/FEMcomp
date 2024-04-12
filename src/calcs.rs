extern crate nalgebra as na;
extern crate nalgebra_sparse as na_sparse;

use crate::vals::{Element, Obj, PhysGeo};

use na::Matrix6;
use na_sparse::{convert::serial::convert_coo_dense,CooMatrix};

impl Element {
    fn c_localc_st(&self, pgs: &Vec<PhysGeo>) -> Matrix6<f32> {
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

        let result: Matrix6<f32> = Matrix6::from_iterator(
            [
                rs[0], 0.0, 0.0, rs[1], 0.0, 0.0, 0.0, sz[0], sz[1], 0.0, sz[2], sz[3], 0.0, sz[4],
                sz[5], 0.0, sz[6], sz[7], rs[2], 0.0, 0.0, rs[3], 0.0, 0.0, 0.0, sz[8], sz[9], 0.0,
                sz[10], sz[11], 0.0, sz[12], sz[13], 0.0, sz[14], sz[15],
            ]
            .into_iter(),
        );
        result
    }
    fn c_cos_matrix(&self) -> Matrix6<f32> {
        let result = Matrix6::from_row_iterator(
            [
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
            ]
            .into_iter(),
        );
        result
    }
    pub(crate) fn c_glob_st(&self, pgs: &Vec<PhysGeo>) -> Matrix6<f32> {
        (self.c_cos_matrix().transpose() * self.c_localc_st(pgs)) * self.c_cos_matrix()
    }
}

impl Obj {
    pub fn c_glob(&self) {
        let mut result: CooMatrix<f32> =
            CooMatrix::zeros(self.elements.len() * 3, self.elements.len() * 3);

        for el in &self.elements {
            let el_r: Matrix6<f32> = el.c_glob_st(&self.physgeos);
            
            let mut nc = 0;
            let mut nr = 0;
            // Проверка на то если индекс в result уже занят
            for i in 0..3 {
                for j in 0..3 {
                    nc = el.node_b_id * 3 + i;
                    nr = el.node_b_id * 3 + j;
                    result.push(nr, nc, el_r[i * 6 + j]);
                    nc = el.node_e_id * 3 + i;
                    nr = el.node_b_id * 3 + j;
                    result.push(nr, nc, el_r[(i + 3) * 6 + j]);
                    nc = el.node_b_id * 3 + i;
                    nr = el.node_e_id * 3 + j;
                    result.push(nr, nc, el_r[i * 6 + j + 3]);
                    nc = el.node_e_id * 3 + i;
                    nr = el.node_e_id * 3 + j;
                    result.push(nr, nc, el_r[(i + 3) * 6 + j + 3]);
                }
            }

        }
        print!("{}", result.ncols()-3);
        print!("{}", convert_coo_dense(&result));
    }
}
