#![allow(warnings)] 

use std::{fs::{File, create_dir_all}, io::{Write,BufReader, self}, path::{PathBuf, Path}};
use dirs;
use std::env::var;
const MSG_NO_SYSTEM_CONFIG_DIR: &str = "no system config directory detected";

fn saveany<T>(x: T)->T {
    // do something with x"
    // return 
    x
}
// pub struct prefstore {
// 	/// App name under which app settings are typically to be stored.
// 	pub app: String,
// }
//  pub enum prefstore{
//     Name(String)
// }
// impl name for prefstore{
    
// }

pub fn setname(appname:String)->String{
    appname
}
// impl ems<String> for String{}
// trait name{
//     fn savepreference<T: ToString>(key: impl Into<String>,value:T);
//     fn default_name(filename:String) -> String;
//     fn config_path(filename:impl Into<String>) -> PathBuf;
//     fn getpreference<T:ToString>(key:impl Into<String>,defvalue:T)->String;
//     // APPNAME:String;
//  }
 
 
//  pub struct setname{
 
//  }
//  impl name for setname{
//  const fn setname(appn:String){
    
//  }
// pub struct prefstore {
// 	/// App name under which app settings are typically to be stored.
// 	pub app: String
// }
// impl prefstore {

//     pub fn setappname(nameofapp:String){
//         app=nameofapp
//     }
// }

pub fn savepreference<T: ToString>(app_name:impl Into<String>,key: impl Into<String>,value:T){
    let key=key.into();
    let app_name=app_name.into();
    // println!("{}",config_path(key.into()).to_string_lossy());
    //create new file named key and save value
    create_dir_all(&config_path(&app_name,&key).parent().expect("Cannot find some path to create config")).expect("cannot create dirs necessary to create config");
    write!(File::create(&config_path(&app_name,&key)).expect("Cannot create file."), "{}", value.to_string());
    // println!("saved successfully to {}",config_path(&app_name,&key).to_string_lossy())
}

fn default_name(filename:String) -> String {
    format!("{}.txt", filename).to_lowercase()
}

fn config_path(app_name:&String,filename:impl Into<String>) -> PathBuf {

    let app= var("CARGO_PKG_NAME").unwrap_or_else(|_| env!("CARGO_PKG_NAME").to_string());
    
    match(dirs::config_dir()){
        Some(system_config_dir) =>{
            system_config_dir
                    .join(app_name)
                    .join(default_name(filename.into()))
        },
        None => {
           PathBuf::new()
            // io::Error::new(io::ErrorKind::NotFound, MSG_NO_SYSTEM_CONFIG_DIR)
        },
    }
}

pub fn getpreference<T:ToString>(app_name:impl Into<String>,key:impl Into<String>,defvalue:T)->String{
    //Try to open file named key, If not found return defvalue
    use io::Read;
    let key =key.into();
    let app_name =app_name.into();
    // let defvalue=defvalue.into();
				match(File::open(&config_path(&app_name,&key))){
                    Ok(mut file) => {
                        let mut buf = String::new();
                        file.read_to_string(&mut buf).expect("Cannot read to string");
                        buf
                    },
                    Err(_) => {
                        savepreference(app_name,&key, &defvalue.to_string());
                        defvalue.to_string()
                    },
                }
}

// }

pub trait ems<T>{
    fn tobool(self)->bool;
    fn toi32(self)->i32;
}

impl ems<String> for String{
    fn tobool(self)->bool{
        let truth_value: bool = match self.as_str() {
            "true" => true,
            "t" => true,
            "false" => false,
            "f" => false,
            // ... any other cases you want
            _ => false  // Or whatever appropriate default value or error.
        };
        truth_value
        }
    fn toi32(self)->i32{
        self.parse::<i32>().unwrap()
    }
    
} 