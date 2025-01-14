use crate::config::Config;
use std::f64::consts::PI;
mod field_boundary_conditions;
use hdf5::{File, Group};

pub struct Field {
    name:String,
    pub field_values:Vec<f64>,
    pub config:Config
}

impl Field {
    
    pub fn new(name:String,config:Config) -> Field {
        let field_values = vec![0.;config.NX+4];


        Field {name:name, field_values:field_values,config:config}
    }

    pub fn save_h5(&self,t:f64) {

        let file = File::open_rw(self.config.file_path_h5.clone()).unwrap();
        let group: Group;
        if file.link_exists(&self.name) {
            // Open the group if it already exists
            group = file.group(&self.name).unwrap();
        } else {
            // Create the group if it does not exist
            group = file.create_group(&self.name).unwrap();
        }
        //let group: Group = file.create_group(&self.name).unwrap();
        let dataset = group.new_dataset::<f64>().shape([self.config.NX+4]).create(t.to_string().as_str()).unwrap();
    
    
        dataset.write(&self.field_values);
        file.close();
    }

    pub fn gradient(&self, gradient_field:&mut Field ){
        for ix in 3..self.config.NX+1 {
            gradient_field.field_values[ix] = -(self.field_values[ix+1] - self.field_values[ix-1] )*self.config.dxi*0.5;
        }
        gradient_field.field_values[2] = -(self.field_values[3] - self.field_values[2] )*self.config.dxi;
        gradient_field.field_values[self.config.NX+2] = -(self.field_values[self.config.NX+2] - self.field_values[self.config.NX+1] )*self.config.dxi;
    }

    fn tolerance_limit(&self, forcing_vector:&Vec<f64>) -> f64 {
        let mut tol = 0. ;
        let mut R;
        for i in 2..self.config.NX+2 {
            R = 1./12. * (forcing_vector[i+1] - 2.*forcing_vector[i] + forcing_vector[i-1]).abs();
            if R>tol {
                tol = R;
            }
        }
        tol
    }

    fn has_converged(&self,forcing_vector:&Vec<f64>) -> bool {
        // let converged = true;

        let tol = self.tolerance_limit(&forcing_vector);
        let mut R :f64;
        for i in 3..self.config.NX+1 {
            R  = (self.field_values[i+1] - 2.* self.field_values[i] + self.field_values[i-1])*self.config.dxi*self.config.dxi - forcing_vector[i];
            R = R.abs();
            if R>tol {
                return false;
            }
        }
        true


  
    }


    pub fn solve_potential(&mut self,forcing_vector:Vec<f64>) {
        // NOT WORKING YET

        //let rho = ( (PI/self.config.NX as f64).cos() +self.config.DX*self.config.DX/DY/DY) * (PI/NY as f64).cos() / (1. + self.config.DX*self.config.DX/DY/DY);
        //let w = 2./ (1. + (1.-rho.powi(2)).sqrt());
        let w = 2./ (1. + (1.-(PI*self.config.DX).sin()));
        let a = 2./(self.config.DX*self.config.DX) ;
        let mut converged:bool = self.has_converged(&forcing_vector);

        while !converged {
            self.boundary_conditions();
            for i in 3..(self.config.NX+1) {
                self.field_values[i] = (1.-w)*self.field_values[i] + w/a * (-forcing_vector[i] + (self.field_values[i-1] + self.field_values[i+1])*self.config.dxi*self.config.dxi) ;
                
            }
            converged = self.has_converged(&forcing_vector);
        }
    }
}
