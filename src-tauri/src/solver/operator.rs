use std::f64::consts::PI;
use crate::solver::reconstruction;
use crate::Element;
use crate::config::ConfigCollision;
mod boundary_conditions_element;


type FnPtr = fn(element:&mut Element,DT:f64);

pub struct Operator {
    execute: FnPtr
}

pub struct ListOperators {
    commands: Vec<Operator>,
}

impl ListOperators {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }
    pub fn add_operator(&mut self, execute: FnPtr) {
        self.commands.push(Operator { execute });
    }

    pub fn execute(&self, elements: &mut Vec<Element>,DT:f64) {
        for operator in &self.commands {

            for element in elements.iter_mut() {
                (operator.execute)(element,DT);
            }
        } 
    }

}


pub fn bgk_collision(element:&mut Element,DT:f64,config_collision:ConfigCollision) {
    
    let temperature = element.temperature();
    let density = element.density();
    let mut maxwell = 0.;
    let mut v = 0.;
    let mut temperature_inter = 0.;

    for ix in 2..element.config.NX+2 {
        for iv  in 2..element.config.NV+2 {
            v = -element.lv + ((iv-2) as f64) * element.dv;
            temperature_inter = temperature[ix]/(density[ix]+1.0e-20);
            maxwell = 1./(2.*PI*(temperature_inter+1.0e-20)).sqrt() * (-v*v*0.5/(temperature_inter+1.0e-20)).exp();
            element.element_grid[ix][iv] = config_collision.frequency_collision * (density[ix]*maxwell -element.element_grid[ix][iv]);

        }
    }
    
}

pub fn x_advection(element:&mut Element,DT:f64) {
    
    let cached_grid = element.element_grid.clone();
    

    let mut v1;
    let mut calc_ind:isize;
    let mut start_ind:usize ;
    let mut th ;


    for ix in 2..element.config.NX+2 {
        for iv  in 2..element.config.NV+2 {

            v1 = -element.lv + ( iv as f64)*element.dv;
            
            calc_ind = ix as isize- (v1*DT*element.config.dxi ).ceil() as isize;
            th = ((v1*DT*element.config.dxi).ceil() as isize) as f64 - v1*DT*element.config.dxi;

            if calc_ind <= 2 {
                element.element_grid[ix][iv] = element.boundary_conditions("lx",ix,iv);
            }
            else if calc_ind >= ((element.config.NX+2) as isize) {
                element.element_grid[ix][iv] = element.boundary_conditions("ux",ix,iv);
            }
            else {
                
            start_ind = calc_ind as usize;

            element.element_grid[ix][iv] = reconstruction::CWENO(th,cached_grid[start_ind-1][iv],cached_grid[start_ind][iv],cached_grid[start_ind+1][iv],cached_grid[start_ind+2][iv],
                                                                element.config.epsi,element.config.DX);
            }

        }
    }
}

pub fn v_advection(element:&mut Element,DT:f64, elec_field: &Vec<f64>) {
    
    let cached_grid = element.element_grid.clone();
    let dvi:f64 = 2.* element.lv/(element.config.NV as f64);

    let mut v1;
    let mut start_ind ;
    let mut calc_ind:isize;
    let mut th ;



    for ix in 2..element.config.NX+2 {
        for iv  in 2..element.config.NV+2 {

            v1 = -( element.z_charge as f64) *elec_field[ix]/element.mu;

            calc_ind = iv as isize- (v1*DT*element.dv ).ceil() as isize;
            th = ((v1*DT*element.dv).ceil() as isize) as f64 - v1*DT*element.dv;

            if calc_ind <= 2 {
                element.element_grid[ix][iv] = element.element_grid[ix][3] ;//element.boundary_conditions("lv",ix,iv);
            }
            else if calc_ind >= ((element.config.NV+2) as isize) {
                element.element_grid[ix][iv] = element.element_grid[ix][element.config.NV+1] ;//element.boundary_conditions("uv",ix,iv);
            }
            else {
                
            start_ind = calc_ind as usize;

            element.element_grid[ix][iv] = reconstruction::CWENO(th, cached_grid[ix][start_ind-1], cached_grid[ix][start_ind], cached_grid[ix][start_ind+1], cached_grid[ix][start_ind+2], element.config.epsi, element.dv);
            }

        }
    }
}
