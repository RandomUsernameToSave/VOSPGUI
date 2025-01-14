use crate::Element;

impl Element {
    pub fn boundary_conditions(&self, l:&str,ix:usize,iv:usize) -> f64 {

        if l == "lx" {
            if self.space_boundary_0 == "zero" {
                return 0.
            }

            else if self.space_boundary_0 == "neumann" {
                return self.element_grid[4][iv]
            }
            else if self.space_boundary_0 == "periodic" {
                return self.element_grid[self.config.NX+2][iv] // 2 NX+2
            }
        }
        else if l == "ux" {
            if self.space_boundary_lx == "zero" {
                return 0.
            }
            else if self.space_boundary_lx == "neumann" {
                return self.element_grid[self.config.NX][iv]
            }
            else if self.space_boundary_lx == "periodic" {
                return self.element_grid[2][iv]
            }

        }
        0.
    }
}