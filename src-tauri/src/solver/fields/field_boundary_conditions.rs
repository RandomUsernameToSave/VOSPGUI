
use super::Field;


impl Field {
    pub fn boundary_conditions(&mut self) {
        if self.config.field_bc_0== "zero" {
            self.field_values[0] = 0. ;
        }
        if self.config.field_bc_0== "neumann" {
            self.field_values[0] = self.field_values[2] ;
        } 
        if self.config.field_bc_lx == "zero" {
            self.field_values[self.config.NX-1] = 0.  ;
        }
        if self.config.field_bc_lx == "neumann" {
            self.field_values[self.config.NX-1] = self.field_values[self.config.NX-2]  ;
        }
    }
}
