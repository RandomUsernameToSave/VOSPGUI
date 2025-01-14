use super::Element;

impl Element {
    pub fn density(&self) -> Vec<f64> {
        let mut density_vec = vec![0.;self.config.NX+4];

        for ix in 2..self.config.NX+2 {
            for iv in 2..self.config.NV+2 {
                density_vec[ix] += self.element_grid[ix][iv]* self.dv;
            }
        }

        density_vec
    }
}