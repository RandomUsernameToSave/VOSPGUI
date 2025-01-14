use crate::Element;

impl Element {
    pub fn boundary_conditions(&self, l:&str,ix:usize,iv:usize) -> f64 {

        if l == "lx" {
            if self.space_boundary_0 == "zero" {
                return 0.
            }

            else if self.space_boundary_0 == "neumann" {
                return self.element_grid[ix+1][iv]
            }
            else if self.space_boundary_0 == "periodic" {
                return self.element_grid[self.config.NX-2][iv]
            }
        }
        else if l == "ux" {
            if self.space_boundary_lx == "zero" {
                return 0.
            }
            else if self.space_boundary_lx == "neumann" {
                return self.element_grid[ix-1][iv]
            }
            else if self.space_boundary_lx == "periodic" {
                return self.element_grid[2][iv]
            }

        }
        0.
    }
}