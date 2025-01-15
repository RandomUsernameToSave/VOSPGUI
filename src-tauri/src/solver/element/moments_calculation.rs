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

    pub fn mean_velocity(&self) -> Vec<f64>  {
        let mut mean_velocity_vec = vec![0.;self.config.NX+4];
        let mut v = 0.;
        for ix in 2..self.config.NX+2 {
            for iv in 2..self.config.NV+2 {
                v = -self.lv + ((iv-2) as f64) * self.dv;
                mean_velocity_vec[ix] += self.element_grid[ix][iv]* v * self.dv;
            }
        }

        mean_velocity_vec
    }

    pub fn temperature(&self) -> Vec<f64>  {
        let mut temperature_vec = vec![0.;self.config.NX+4];
        let mean_velocity_vec = self.mean_velocity();
        let mut v = 0.;
        for ix in 2..self.config.NX+2 {
            for iv in 2..self.config.NV+2 {
                v = -self.lv + ((iv-2) as f64) * self.dv;
                temperature_vec[ix] += self.element_grid[ix][iv]* (v - mean_velocity_vec[ix]) * (v - mean_velocity_vec[ix]) * self.dv;
            }
        }

        temperature_vec
    }
}