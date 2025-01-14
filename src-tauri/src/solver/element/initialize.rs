
use std::f64::consts::PI;
use super::Element;


impl Element {
    pub fn initialize_grid(&mut self) {
        
        if self.init_cond == "zero" {
            for i in 0..self.config.NX+4 {
                for j in 0..self.config.NV+4 {
                    self.element_grid[i][j] = 0.;
                }
            }
        }

        else if self.init_cond == "maxwellian" {
            let mut v:f64; 
            for ix in 0..self.config.NX+4 {
                for iv in 0..self.config.NV+4 {
                    v = -self.lv + (iv as f64) * self.dv;
                    self.element_grid[ix][iv] = (self.mu/2./PI).sqrt() * (-0.5*self.mu*v*v).exp() ;
                }
            }
        }
        else if self.init_cond == "half-maxwellian" {
            let mut v:f64; 
            for ix in 0..self.config.NX+4 {
                for iv in 0..self.config.NV+4 {
                    v = -self.lv + (iv as f64) * self.dv;
                    if 2*ix < self.config.NX {
                    self.element_grid[ix][iv] = (self.mu/2./PI).sqrt() * (-0.5*self.mu*v*v).exp() ;
                    }
                    else {
                        self.element_grid[ix][iv] = 0.
                    }
                }
            }
        }

    }
}
