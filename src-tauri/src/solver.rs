pub mod element;
pub mod operator;
pub mod reconstruction;
mod fields;

use element::{Element,Shareable_element};
use crate::config::Config;
use crate::n_save;
use tauri::{AppHandle, Emitter};
use operator::{ListOperators,Operator};

use std::{fs, vec};
use std::path::Path;
use hdf5::File;
extern crate chrono;


pub struct Solver {
    elements:Vec<Element>,
    electric_field:fields::Field,
    potentiel_field:fields::Field,
    pub config: Config,
    list_operators : ListOperators,
}


impl Solver {


    pub fn new(config:Config)-> Solver { 
        let elements    = vec![];
        
        //let now:String = Utc::now().format("%Y%m%d_%H_%M_%S").to_string();
        //let file_path = format!("DATA/{}.h5", now); 

        
        //let file = File::create("VOSP.h5").unwrap();
        let electric_field  = fields::Field::new(String::from("electric field"),config.clone());
        let potentiel_field  = fields::Field::new(String::from("potential field"),config.clone());
        let list_operators = ListOperators::new();
        Solver{elements:elements,electric_field:electric_field,potentiel_field,config:config,list_operators:list_operators}
    }

    pub fn set_config(&mut self,config:Config) {
        self.config = config;
    }

    pub fn add_element (&mut self,element:Element) {
        self.elements.push(element);
    }

    pub fn init(&mut self) {
        self.electric_field.field_values = vec![0.;self.config.NX+4];
        self.potentiel_field.field_values = vec![0.;self.config.NX+4];
        self.electric_field.config = self.config.clone();
        self.potentiel_field.config = self.config.clone();
        for element in self.elements.iter_mut() {
            element.config = self.config.clone();
            element.init(); // on initalize tous les champs de vecteurs des elements

        }
        // Add operators to the init 

        //self.list_operators.add_operator(operator::x_advection);

    }


    fn calculate_charge_density(&self) -> Vec<f64> {
        let mut charge_density = vec![0.;self.config.NX+4]; 
        for element in self.elements.iter() {
            for i in 0..self.config.NX+4 {
                charge_density[i] += (element.z_charge as f64) * element.density()[i] / (self.config.lambda*self.config.lambda);
            }
        }
        charge_density
    }

    pub fn next_step(&mut self) {
        
        // Spatial advection
        for element in self.elements.iter_mut() {
            operator::x_advection( element,self.config.DT);
        }

        // Calculate the advection in velocity space
        let charge_density = self.calculate_charge_density();
        self.potentiel_field.solve_potential(charge_density);
        self.potentiel_field.gradient(&mut self.electric_field);

        for element in self.elements.iter_mut() {
            operator::v_advection( element,self.config.DT, &self.electric_field.field_values);
        }

        // Collisions operators

        self.list_operators.execute( &mut self.elements,self.config.DT/2.);

    }

    pub fn run(&mut self, app: AppHandle) {

        let mut t = self.config.DT;
        let mut k_iter: u32 = 1;
        println!("Solver is starting");
        let mut progress = 0;

        let file = File::create(self.config.file_path_h5.clone()).unwrap();
        file.close();
        while t < self.config.end_time {
            if (t/self.config.end_time*100.) as usize > progress  {
                progress = (t/self.config.end_time*100.) as usize;
                println!("progress : {}",progress);
                app.emit("progress-update", &progress).unwrap();
            }
            
            t += self.config.DT; 
            self.next_step();
            k_iter += 1;

            if k_iter% self.config.n_save == 0 {
                for element in self.elements.iter() {
                    element.save_h5( t);
                }
                self.electric_field.save_h5( t);
                self.potentiel_field.save_h5( t);
            }

        }
    }

    pub fn print(&self) {
        for element in self.elements.iter() {
            element.print();
        }
    }
    pub fn share_elements(&self) -> Vec<Shareable_element>{
        let mut vec_share: Vec<Shareable_element> = Vec::new();
        for element in self.elements.iter() {
            vec_share.push(element.to_share());
        }
        vec_share
    }

}


