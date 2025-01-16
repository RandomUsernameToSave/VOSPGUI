use std::f64::consts::PI;

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

    let mut betal ;
    let mut betar;
    let mut betac;

    let mut al ;
    let mut ar ;
    let mut ac:f64;
    let mut r0:f64;
    let mut r1:f64;
    let mut r2:f64;
    let mut r0p:f64;
    let mut r1p:f64;
    let mut r2p:f64;
    let mut t0:f64;
    let mut t1:f64;
    let mut t2:f64;
    let mut q:f64;


    for ix in 2..element.config.NX+2 {
        for iv  in 2..element.config.NV+2 {

            v1 = -element.lv + ( iv as f64)*element.dv;
            
            calc_ind = ix as isize- (v1*DT*element.config.dxi ).ceil() as isize;
            //calc_ind -= 1;
            th = ((v1*DT*element.config.dxi).ceil() as isize) as f64 - v1*DT*element.config.dxi;

            if calc_ind <= 2 {
                element.element_grid[ix][iv] = element.boundary_conditions("lx",ix,iv);
            }
            else if calc_ind >= ((element.config.NX+2) as isize) {
                element.element_grid[ix][iv] = element.boundary_conditions("ux",ix,iv);
            }
            else {
                
            start_ind = calc_ind as usize;


            betal =  (cached_grid[start_ind][iv] - cached_grid[start_ind-1][iv]).powi(2);
            betar= (cached_grid[start_ind+1][iv] - cached_grid[start_ind][iv]).powi(2);
            betac = ((cached_grid[start_ind+1][iv] - 2. * cached_grid[start_ind][iv] + cached_grid[start_ind-1][iv]).powi(2)) * 13./3. 
                    + 0.25 * (( cached_grid[start_ind+1][iv] - cached_grid[start_ind-1][iv]).powi(2));

            al = 0.25 / ((element.config.epsi + betal).powi(2));
            ar = 0.25 / ((element.config.epsi + betar).powi(2));
            ac = 0.5 / ((element.config.epsi + betac).powi(2));

            betar = ar/ (ar+ac+al);
            betal = al/ (ar+ac+al);
            betac = ac/ (ar+ac+al);

            r0 = cached_grid[start_ind][iv] - 1./12. * betac * (cached_grid[start_ind+1][iv] - 2.* cached_grid[start_ind][iv] + cached_grid[start_ind-1][iv]);
            r1 =    betal * element.config.dxi * (cached_grid[start_ind][iv] - cached_grid[start_ind-1][iv])  + 
                    betar * element.config.dxi * (cached_grid[start_ind+1][iv] - cached_grid[start_ind][iv]) + 
                    betac * 0.5*element.config.dxi * (cached_grid[start_ind+1][iv] - cached_grid[start_ind-1][iv]);
            r2 = 2.* betac * element.config.dxi*element.config.dxi*((cached_grid[start_ind+1][iv] - 2.* cached_grid[start_ind][iv] + cached_grid[start_ind-1][iv]));
            

            // SECOND PASS
            // 
            // SECOND PASS 
            
            start_ind += 1;

            betal =  (cached_grid[start_ind][iv] - cached_grid[start_ind-1][iv]).powi(2);
            betar= (cached_grid[start_ind+1][iv] - cached_grid[start_ind][iv]).powi(2);
            betac = 13./3. * ((cached_grid[start_ind+1][iv] - 2. * cached_grid[start_ind][iv] + cached_grid[start_ind-1][iv]).powi(2)) + 0.25 * (( cached_grid[start_ind+1][iv] - cached_grid[start_ind-1][iv]).powi(2));

            al = 0.25 / ((element.config.epsi + betal).powi(2));
            ar = 0.25 / ((element.config.epsi + betar).powi(2));
            ac = 0.5 / ((element.config.epsi + betac).powi(2));

            betar = ar/ (ar+ac+al);
            betal = al/ (ar+ac+al);
            betac = ac/ (ar+ac+al);

            r0p = cached_grid[start_ind][iv] - 1./12. * betac * (cached_grid[start_ind+1][iv] - 2.* cached_grid[start_ind][iv] + cached_grid[start_ind-1][iv]);
            r1p =    betal * element.config.dxi * (cached_grid[start_ind][iv] - cached_grid[start_ind-1][iv])  + 
                    betar * element.config.dxi * (cached_grid[start_ind+1][iv] - cached_grid[start_ind][iv]) + 
                    betac * 0.5*element.config.dxi * (cached_grid[start_ind+1][iv] - cached_grid[start_ind-1][iv]);
            r2p = 2.* betac * element.config.dxi*element.config.dxi*((cached_grid[start_ind+1][iv] - 2.* cached_grid[start_ind][iv] + cached_grid[start_ind-1][iv]));

            // LAST RECONSTRUCTION 
            //
            // LAST RECONSTRUCTION 

            t0 =(1.-th)*r0 + th*r0p;
            t1 = element.config.DX* th*(1.-th) * (0.5*r1 - 0.5 * r1p);
            q = 3.*th - 6.*th*th + 4. * th*th*th;
            t2 = element.config.DX*element.config.DX /24.* ( (1.-q)*r2 +q*r2p);
            element.element_grid[ix][iv] = t0+t1+t2;
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

    let mut betal ;
    let mut betar;
    let mut betac ;

    let mut al ;
    let mut ar ;
    let mut ac:f64;
    let mut r0:f64;
    let mut r1:f64;
    let mut r2:f64;
    let mut r0p:f64;
    let mut r1p:f64;
    let mut r2p:f64;
    let mut t0:f64;
    let mut t1:f64;
    let mut t2:f64;
    let mut q:f64;


    for ix in 2..element.config.NX+2 {
        for iv  in 2..element.config.NV+2 {

            v1 = -( element.z_charge as f64) *elec_field[ix]/element.mu;
            calc_ind = iv as isize- (v1*DT*element.dv ).ceil() as isize;
            //calc_ind -= 1;
            th = ((v1*DT*element.dv).ceil() as isize) as f64 - v1*DT*element.dv;

            if calc_ind <= 2 {
                element.element_grid[ix][iv] = element.element_grid[ix][3] ;//element.boundary_conditions("lv",ix,iv);
            }
            else if calc_ind >= ((element.config.NV+2) as isize) {
                element.element_grid[ix][iv] = element.element_grid[ix][element.config.NV+1] ;//element.boundary_conditions("uv",ix,iv);
            }
            else {
                
            start_ind = calc_ind as usize;


            betal =  (cached_grid[ix][start_ind] - cached_grid[ix][start_ind-1]).powi(2);
            betar= (cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind]).powi(2);
            betac = 13./3. * ((cached_grid[ix][start_ind+1] - 2. * cached_grid[ix][start_ind] + cached_grid[ix][start_ind-1]).powi(2)) + 0.25 * (( cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind-1]).powi(2));

            al = 0.25 / ((element.config.epsi + betal).powi(2));
            ar = 0.25 / ((element.config.epsi + betar).powi(2));
            ac = 0.5 / ((element.config.epsi + betac).powi(2));

            betar = ar/ (ar+ac+al);
            betal = al/ (ar+ac+al);
            betac = ac/ (ar+ac+al);

            r0 = cached_grid[ix][start_ind] - 1./12. * betac * (cached_grid[ix][start_ind+1] - 2.* cached_grid[ix][start_ind] + cached_grid[ix][start_ind-1]);
            r1 =    betal * (cached_grid[ix][start_ind] - cached_grid[ix][start_ind-1]) /element.dv + 
                    betar/element.dv * (cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind]) + 
                    0.5*betac/element.dv * (cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind-1]);
            r2 = 2.* betac /element.dv/element.dv*((cached_grid[ix][start_ind+1] - 2.* cached_grid[ix][start_ind] + cached_grid[ix][start_ind-1]));
            

            // SECOND PASS
            // 
            // SECOND PASS 
            
            start_ind += 1;

            betal =  (cached_grid[ix][start_ind] - cached_grid[ix][start_ind-1]).powi(2);
            betar= (cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind]).powi(2);
            betac = 13./3. * ((cached_grid[ix][start_ind+1] - 2. * cached_grid[ix][start_ind] + cached_grid[ix][start_ind-1]).powi(2)) + 0.25 * (( cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind-1]).powi(2));

            al = 0.25 / ((element.config.epsi + betal).powi(2));
            ar = 0.25 / ((element.config.epsi + betar).powi(2));
            ac = 0.5 / ((element.config.epsi + betac).powi(2));

            betar = ar/ (ar+ac+al);
            betal = al/ (ar+ac+al);
            betac = ac/ (ar+ac+al);

            r0p = cached_grid[ix][start_ind] - 1./12. * betac * (cached_grid[ix][start_ind+1] - 2.* cached_grid[ix][start_ind] + cached_grid[ix][start_ind-1]);
            r1p =    betal * (cached_grid[ix][start_ind] - cached_grid[ix][start_ind-1]) /element.dv + 
                    betar/element.dv * (cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind]) + 
                    0.5*betac/element.dv * (cached_grid[ix][start_ind+1] - cached_grid[ix][start_ind-1]);
            r2p = 2.* betac /element.dv/element.dv*((cached_grid[ix][start_ind+1] - 2.* cached_grid[ix][start_ind] + cached_grid[ix][start_ind-1]));

            // LAST RECONSTRUCTION 
            //
            // LAST RECONSTRUCTION 

            t0 =(1.-th)*r0 + th*r0p;
            t1 = element.config.DX* th*(1.-th) * (0.5*r1 - 0.5 * r1p);
            q = 3.*th - 6.*th*th + 4. * th*th*th;
            t2 = element.config.DX*element.config.DX /24.* ( (1.-q)*r2 +q*r2p);
            element.element_grid[ix][iv] = t0+t1+t2;
            }

        }
    }
}
