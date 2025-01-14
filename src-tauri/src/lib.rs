use tauri_plugin_opener::init;
use std::sync::Mutex;
mod solver;
mod config;
mod command;
use config::Config;
use solver::{element::Element, Solver};
use hdf5::{File, Group};
use tauri::{Builder, Manager,State,AppHandle, Emitter};
use solver::element::Shareable_element;
use command::{addConfig,list_all_files,read_h5,read_dataset_h5,select_h5_dataset};

pub const NX:usize = 256;
pub const NV:usize = 100;
pub const LX:f64 = 1.;
//pub const LV:f64 = 1.;
pub const DT:f64 = 0.000005;
pub const DX:f64 = LX/(NX as f64);
//pub const DV:f64 = LV/(NV as f64);
pub const dxi:f64 = 1./DX ;
pub const epsi:f64 = 1.0e-20;
pub const T:f64 = 0.01;
pub const lambda:f64=0.01;

pub const field_bc_left:u32=0;
pub const field_bc_right:u32=1;
pub const n_save:u32 = 200;



// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn my_command(z_charge: isize) {
    println!("Received Z_charge: {}", z_charge);
}

#[tauri::command]
fn my_testcommand() {
    println!("Test");
}

#[tauri::command]
fn new_element(state: State<'_, Mutex<Solver>>,element_name:String,lv:f64, init_cond:u32,z_charge:i32,mu:f64,right_boundary:u32,left_boundary:u32) {
    let mut solver = state.lock().unwrap(); 

    let element = Element::new(element_name,lv,init_cond,z_charge,mu,left_boundary,right_boundary,solver.config.clone());
    solver.add_element(element);
    solver.print();
}

#[tauri::command]
fn get_elements(state: State<'_, Mutex<Solver>>)-> Vec<Shareable_element> {
    let solver = state.lock().unwrap(); 
    solver.share_elements()
}


#[tauri::command]
async fn run_solver(app: AppHandle , state: State<'_, Mutex<Solver>>) -> Result<(), ()> {
    let mut solver = state.lock().unwrap(); 
    solver.init();
    solver.run(app);
    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("here");
    let config = Config::new(NX,NV,LX,DT,T,epsi,lambda,field_bc_left,field_bc_right,n_save);

    tauri::Builder::default().setup(|app| {

            app.manage(Mutex::new(Solver::new(config)));
            
            Ok(())

        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,
                                                    new_element,
                                                    get_elements,
                                                    addConfig,
                                                    run_solver,
                                                    list_all_files,
                                                    read_h5,
                                                    read_dataset_h5,
                                                    select_h5_dataset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
