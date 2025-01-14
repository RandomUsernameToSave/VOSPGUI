mod initialize;
mod moments_calculation;
use std::{fmt::format, ptr::null};


use crate::config::Config;
use hdf5::{File, Group};
use ndarray::Array2;
use serde::{Serialize, Deserialize};

#[derive(serde::Serialize)]
pub struct Element {
    element_name:String,
    pub element_grid:Vec<Vec<f64>>,
    pub lv:f64,
    pub dv:f64,
    pub z_charge:i32,
    pub mu:f64,
    init_cond:u32,
    pub lx_boundary:u32,
    pub ux_boundary:u32,
    pub config:Config
}

impl Element {
    pub fn new(element_name:String, lv:f64,init_cond:u32,z_charge:i32,mu:f64,lx_boundary:u32,ux_boundary:u32,config:Config) -> Element {
        let element_grid = vec![vec![0.;config.NV];config.NX]; 
        let dv = 2.*lv/config.NV as f64;

        
        Element{element_name:element_name,element_grid:element_grid,lv:lv,dv:dv  , z_charge:z_charge,  mu:mu,   
            init_cond:init_cond,lx_boundary:lx_boundary,ux_boundary:ux_boundary,config:config}
    }

    pub fn init(&mut self) {
        self.element_grid = vec![vec![0.;self.config.NV];self.config.NX]; 
        self.initialize_grid();
    }



    pub fn save_h5(&self, t:f64) {

        fn vec_to_array<T: Clone>(v: &Vec<Vec<T>>) -> Array2<T> {
            if v.is_empty() {
                return Array2::from_shape_vec((0, 0), Vec::new()).unwrap();
            }
            let nrows = v.len();
            let ncols = v[0].len();
            let mut data = Vec::with_capacity(nrows * ncols);
            for row in v {
                assert_eq!(row.len(), ncols);
                data.extend_from_slice(&row);
            }
            Array2::from_shape_vec((nrows, ncols), data).unwrap()
        }
        let data = vec_to_array(&self.element_grid);

        let file = File::open_rw(self.config.file_path_h5.clone()).unwrap();

        let group: Group;
        if file.link_exists(&self.element_name) {
            // Open the group if it already exists
            group = file.group(&self.element_name).unwrap();
        } else {
            // Create the group if it does not exist
            group = file.create_group(&self.element_name).unwrap();
        }
        //let group: Group = file.create_group(&self.element_name).unwrap();
        let dataset = group.new_dataset::<f64>().shape([self.config.NX,self.config.NV]).create(t.to_string().as_str()).unwrap();
    
        
        //let dataset = group.new_dataset::<f64>().shape([self.config.NX,self.config.NV]).create(t.to_string().as_str()).unwrap();
    
    
        dataset.write(&data);
        file.close();
        //println!("Saved successfully");
    }

    pub fn print(&self) {
        println!("{}", self.element_name);
    }

    pub fn to_share(&self) -> Shareable_element {
        Shareable_element {element_name:self.element_name.clone(),z_charge:self.z_charge,lv:self.lv,mu:self.mu}
    }


}


#[derive(Serialize, Deserialize)]
pub struct Shareable_element {
    element_name:String,
    z_charge:i32,
    lv:f64,
    mu:f64,
}
