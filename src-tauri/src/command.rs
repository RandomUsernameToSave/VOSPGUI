use crate::config::Config;
use crate::solver::Solver;
use std::sync::Mutex;
use hdf5::File;
use tauri::State;
// use tauri::ipc::Response;

use std::fs;

#[tauri::command]
pub fn addConfig(state: State<'_, Mutex<Solver>> , NX:usize,NV:usize,LX:f64,DT:f64,end_time:f64,lambda:f64,field_bc_left:String,field_bc_right:String,n_save:u32 ) {
    let mut solver = state.lock().unwrap(); 
    let epsi = 1.0e-20;
    let config = Config::new(NX, NV, LX, DT, end_time, epsi, lambda, field_bc_left, field_bc_right, n_save);
    // println!("IN CONFIG");
    config.print();
    solver.set_config(config);
    
}

#[tauri::command]
pub fn list_all_files()-> Vec<String> {
    
    
    let path = fs::read_dir("./DATA")
    .unwrap()
    .filter_map(|e| e.ok())
    .map(|e| e.path().to_string_lossy().into_owned())
    .collect::<Vec<String>>();

    path

}

#[tauri::command]
pub fn read_h5(path_h5:String)-> Vec<String> {
    let file = File::open(path_h5).unwrap();

    // Access the root group ("/") of the file
    let root_group = file.group("/").unwrap();
    let object_names = root_group.member_names().unwrap();

    object_names

}

#[tauri::command]
pub fn read_dataset_h5(path_h5:String , group_h5:String)-> Vec<String> {
    let file = File::open(path_h5).unwrap();


    let group = file.group(&group_h5).unwrap();
    let object_names = group.member_names().unwrap();

    object_names

}
#[tauri::command]
pub fn select_h5_dataset(path_h5:String , group_h5:String,dataset:String)-> Vec<Vec<f64>> {
    let file = File::open(path_h5).unwrap();


    let group: hdf5::Group = file.group(&group_h5).unwrap();
    let dataset = group.dataset(&dataset).unwrap();
    let data = dataset.read_2d().unwrap();
    let vec_2d: Vec<Vec<f64>> = data.outer_iter()
    .map(|row| row.to_vec())
    .collect();

    //tauri::ipc::Response::new(vec_2d)
    vec_2d
}
